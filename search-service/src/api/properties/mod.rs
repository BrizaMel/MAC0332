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

use crate::relational::entities::DbSchema;

use crate::database_storage::DatabaseStorage;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Properties {
    attributes: Vec<AttributeInfo>,
    subsets: Vec<HashSet<u8>>,
    operators: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributeInfo {
    name: String,
    data_type: DataType,
    subset_id: u8
}

pub async fn get_filter_properties(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {

    let db_storage = &app_state.db;

    let db_schema_info = db_storage.get_db_schema_info().await.expect("Error retireving Database Schema Information");
    
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

        let operators_vec = Operator::iter().collect::<Vec<_>>();
        let operators = operators_vec.iter().map(|o| o.clone().to_string()).collect();

        let mut attributes : Vec<AttributeInfo> = Vec::new();

        let mut attributes_subsets : Vec<HashSet<u8>> = Vec::new();

        let mut tables_subsets : Vec<HashSet<String>> = Vec::new();
        
        for table in schema_info.tables {

            let mut full_table_name = "".to_string();
            full_table_name.push_str(&table.schema);
            full_table_name.push_str(".");
            full_table_name.push_str(&table.name);

            let table_subset_id = manage_subsets(&full_table_name, &mut tables_subsets, &mut attributes_subsets)
                .expect("Error finding table subset id");

            for attribute in table.attributes{

                let mut full_attr_name = full_table_name.to_string();
                full_attr_name.push_str(".");  
                full_attr_name.push_str(&attribute.name);

                let data_type = db_storage
                    .translate_native_type(&attribute.data_type)
                    .expect(&format!("Error translating data type: {}",attribute.data_type));

                let attribute_info = AttributeInfo::new(full_attr_name,data_type,table_subset_id);
                attributes.push(attribute_info);


                let attribute_idx = (attributes.len() - 1) as u8;
                attributes_subsets[table_subset_id as usize].insert(attribute_idx);

            }       
        }

        let subsets : Vec<HashSet<u8>> = attributes_subsets;

        Self {
            attributes,
            subsets,
            operators
        }
    }

}

fn manage_subsets(table: &str, tables_subsets: &mut Vec<HashSet<String>>, attributes_subsets: &mut Vec<HashSet<u8>>) -> Result<u8,Error> {
    let table_subset_id = find_subset_id_for_table(&table, &tables_subsets)?;

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
fn find_subset_id_for_table(table: &str, table_subsets: &Vec<HashSet<String>>)-> Result<u8,Error> {
    let mut subset_id : u8 = table_subsets.len() as u8;

    let mut idx = 0;
    for subset in table_subsets {
        for table_name in subset{
            // TODO: check if table and table_name are joinable
            // If so, subset_id = idx
            // else, just break
            println!("{:?}",table_name);
            subset_id = idx;
            break;
        }
        idx = idx + 1;
    }

    Ok(subset_id)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Error;
    use crate::database_storage::DatabaseStorage;
   
    use crate::postgres::{PostgresConfig,PostgresStorage};

    async fn aux_get_db_schema(db_storage: &DatabaseStorage) -> Result<DbSchema,Error> {
        let db_schema_info = db_storage.get_db_schema_info().await?;
        Ok(db_schema_info)
    }

    async fn aux_get_storage() -> Result< DatabaseStorage, Error> {
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

    #[tokio::test]
    async fn test_operators_creation() -> Result<(), Error> {

        let db_storage = aux_get_storage().await?;

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
            ]
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_attributes_types_correctness() -> Result<(), Error> {

        let db_storage = aux_get_storage().await?;

        let db_schema_info = aux_get_db_schema(&db_storage).await?;
        
        let properties = Properties::from_schema_info(db_schema_info, &db_storage);

        let data_type_vec = DataType::iter().collect::<Vec<_>>();
        let possible_data_types : Vec<String> = data_type_vec.iter().map(|o| o.clone().to_string()).collect();

        for attribute in properties.attributes {
            assert_eq!(possible_data_types.contains(&attribute.data_type.to_string()),true);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_subsets_correctness() -> Result<(), Error> {

        let db_storage = aux_get_storage().await?;

        let db_schema_info = aux_get_db_schema(&db_storage).await?;
        
        let properties = Properties::from_schema_info(db_schema_info, &db_storage);

        for attribute in properties.attributes {
            assert_eq!(attribute.subset_id,0);
        }

        assert_eq!(properties.subsets.len(),1);

        Ok(())
    }
}

