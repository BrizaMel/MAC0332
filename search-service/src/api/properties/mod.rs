use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use anyhow::Error;

use strum::IntoEnumIterator;

use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::collections::HashSet;

use crate::controller::http::AppState;

use crate::traits::DatabaseOperations;

use crate::query_representation::intermediary::single_command::{DataType,Operator};
use crate::query_representation::intermediary::composite_command::LogicalOperator;

use crate::relational::table_search::TableSearch;
use crate::relational::table_search::entities::TableSearchInfo;

use crate::relational::entities::DbSchema;

use crate::database_storage::DatabaseStorage;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Properties {
    attributes: Vec<AttributeInfo>,
    subsets: Vec<HashSet<u8>>,
    operators: Vec<String>,
    logical_operators: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributeInfo {
    name: String,
    data_type: DataType,
    subset_id: u8
}

pub async fn get_filter_properties(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {

    let db_storage = &app_state.db;

    let db_schema_info = db_storage.get_db_schema_info()
        .await
        .expect("Error retireving Database Schema Information");
    
    let properties = Properties::from_schema_info(db_schema_info, db_storage);

    let json_response = serde_json::json!({
        "status": "success",
        "properties": serde_json::json!(properties),
    });

    Json(json_response)
}


impl AttributeInfo {
    pub fn new(
        name: String,
        data_type: DataType,
        subset_id: u8,
    ) -> Self {
        Self {
            name,
            data_type,
            subset_id,
        }
    }
}

impl Properties {
    pub fn from_schema_info(schema_info: DbSchema, db_storage: &DatabaseStorage) -> Self {

        // TableSearch creation
        let mut tables_search_info: Vec<TableSearchInfo> = Vec::new();
        for table in schema_info.tables.to_owned() {
            let table_search_info = TableSearchInfo::new(table.schema,table.name);
            tables_search_info.push(table_search_info);
        }
        let table_search = TableSearch::new(tables_search_info, schema_info.foreing_keys);


        let mut attributes : Vec<AttributeInfo> = Vec::new();
        let mut attributes_subsets : Vec<HashSet<u8>> = Vec::new();
        let mut tables_subsets : Vec<HashSet<String>> = Vec::new();

        for table in schema_info.tables {

            let full_table_name = format!("{}.{}",&table.schema,&table.name).to_string();

            let table_subset_id = manage_subsets(&full_table_name, &mut tables_subsets,
                &mut attributes_subsets, &table_search).expect("Error finding table subset id");

            for attribute in table.attributes{

                let full_attr_name = format!("{}.{}",full_table_name,&attribute.name).to_string();

                let data_type = db_storage
                    .translate_native_type(&attribute.data_type)
                    .expect(&format!("Error translating data type: {}",attribute.data_type));

                let attribute_info = AttributeInfo::new(full_attr_name,data_type,table_subset_id);
                attributes.push(attribute_info);


                let attribute_idx = (attributes.len() - 1) as u8;
                attributes_subsets[table_subset_id as usize].insert(attribute_idx);

            }       
        }

        let operators = Operator::iter()
            .map(|o| o.clone().to_string())
            .collect();

        let logical_operators = LogicalOperator::iter()
            .map(|o| o.clone().to_string())
            .collect();

        Self {
            attributes,
            subsets: attributes_subsets,
            operators,
            logical_operators
        }
    }

}

fn manage_subsets(
    table: &str,
    tables_subsets: &mut Vec<HashSet<String>>,
    attributes_subsets: &mut Vec<HashSet<u8>>,
    table_search: &TableSearch
) -> Result<u8,Error> {
    let table_subset_id = find_subset_id_for_table(
        &table,
        &tables_subsets,
        &table_search
    )?;

    if table_subset_id >= tables_subsets.len() as u8 {
        let mut new_hashset = HashSet::new();
        new_hashset.insert(table.to_string());
        tables_subsets.push(new_hashset);
    }
    else{
        tables_subsets[table_subset_id as usize].insert(table.to_string());
    }

    if table_subset_id >= attributes_subsets.len() as u8 {
        let new_hashset = HashSet::new();
        attributes_subsets.push(new_hashset);
    }

    Ok(table_subset_id)
}

// Given a table and a list of table sets, find which of these sets (or none of them)
// the table belongs to. A table belongs to a set if it is joinable with the other tables of the set
// The return value is the index of the subset in the HashSet vector.
fn find_subset_id_for_table(
    table: &str,
    table_subsets: &Vec<HashSet<String>>,
    table_search: &TableSearch
)-> Result<u8,Error> {
    let mut subset_id : u8 = table_subsets.len() as u8;

    let mut idx = 0;
    for subset in table_subsets {
        for table_name in subset {
            if are_tables_joinable(table, table_name, table_search)? {
                subset_id = idx;
                return Ok(subset_id);
            }
            break;
        }

        idx = idx + 1;
    }

    Ok(subset_id)
}

fn are_tables_joinable(
    table_a: &str,
    table_b: &str,
    table_search: &TableSearch
) -> Result<bool,Error> {
    let (joinable_tables,_) = table_search
        .path_to(
            table_a.to_string(),
            table_b.to_string()
        )?;

    Ok(joinable_tables.contains(&table_b.to_string()))
}


#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Error;
    use crate::database_storage::DatabaseStorage;
   
    use crate::postgres::{PostgresConfig,PostgresStorage};

    use crate::mysql::{MySQLConfig,MySQLStorage};

    async fn aux_get_db_schema(db_storage: &DatabaseStorage) -> Result<DbSchema,Error> {
        let db_schema_info = db_storage.get_db_schema_info().await?;
        Ok(db_schema_info)
    }

    async fn aux_get_pg_storage() -> Result< DatabaseStorage, Error> {
         let storage = DatabaseStorage::PostgresStorage(
            PostgresStorage::new(
                PostgresConfig::new(
                    "public,movies".into(),
                    "localhost".into(),
                    54329,
                    "search-service".into(),
                    "search-service".into(),
                    "search-service".into()
                )
            ).await?
        );
        Ok(storage)     
    }

    async fn aux_get_mysql_storage() -> Result< DatabaseStorage, Error> {
         let storage = DatabaseStorage::MySQLStorage(
            MySQLStorage::new(
                MySQLConfig::new(
                    "public,movies".into(),
                    "localhost".into(),
                    3306,
                    "searchservice".into(),
                    "searchservice".into(),
                    "searchservice".into()
                )
            ).await?
        );
        Ok(storage)     
    }

    #[tokio::test]
    async fn test_operators_creation() -> Result<(), Error> {

        let dbms_storages = vec![aux_get_pg_storage().await?,aux_get_mysql_storage().await?];

        for db_storage in dbms_storages {

            let db_schema_info = aux_get_db_schema(&db_storage).await?;
            
            let properties = Properties::from_schema_info(db_schema_info, &db_storage);

            assert_eq!(properties.operators,
                vec![
                    "EqualTo".to_string(),
                    "GreaterThan".to_string(),
                    "LessThan".to_string(),
                    "GreaterThanOrEqualTo".to_string(),
                    "LessThanOrEqualTo".to_string(),
                    "NotEqualTo".to_string()
                ],
                "Testing for database_storage: {}", db_storage
            );           
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_logical_operators_creation() -> Result<(), Error> {

        let dbms_storages = vec![aux_get_pg_storage().await?,aux_get_mysql_storage().await?];

        for db_storage in dbms_storages {

            let db_schema_info = aux_get_db_schema(&db_storage).await?;
            
            let properties = Properties::from_schema_info(db_schema_info, &db_storage);

            assert_eq!(properties.logical_operators,
                vec![
                    "AND".to_string(),
                    "OR".to_string(),
                ],
                "Testing for database_storage: {}", db_storage
            );

        }

        Ok(())
    }

    #[tokio::test]
    async fn test_attributes_types_correctness() -> Result<(), Error> {

        let dbms_storages = vec![aux_get_pg_storage().await?,aux_get_mysql_storage().await?];

        for db_storage in dbms_storages {

            let db_schema_info = aux_get_db_schema(&db_storage).await?;
            
            let properties = Properties::from_schema_info(db_schema_info, &db_storage);

            let data_type_vec = DataType::iter().collect::<Vec<_>>();
            let possible_data_types : Vec<String> = data_type_vec
                .iter()
                .map(|o| o.clone().to_string())
                .collect();

            for attribute in properties.attributes {
                assert_eq!(
                    possible_data_types.contains(&attribute.data_type.to_string()),
                    true,
                    "Testing for database_storage: {}", db_storage
                );
            }

        }

        Ok(())
    }

    #[tokio::test]
    async fn test_subsets_correctness() -> Result<(), Error> {

        let dbms_storages = vec![aux_get_pg_storage().await?,aux_get_mysql_storage().await?];

        for db_storage in dbms_storages {

            let db_schema_info = aux_get_db_schema(&db_storage).await?;
            
            let properties = Properties::from_schema_info(db_schema_info, &db_storage);

            for attribute in properties.attributes {
                assert_eq!(attribute.subset_id,0);
            }

            assert_eq!(
                properties.subsets.len(),
                1,
                "Testing for database_storage: {}", db_storage
            );

        }

        Ok(())

    }


}

