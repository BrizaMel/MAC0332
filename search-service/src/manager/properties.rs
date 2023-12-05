use anyhow::Error;

use strum::IntoEnumIterator;

use serde::{Deserialize, Serialize};

use std::{
    sync::Arc,
    collections::HashSet
};

use crate::{
    query_representation::{
        intermediary::{
            single_command::{DataType,Operator},
            composite_command::LogicalOperator
        }
    },
    relational::{
        table_search::TableSearch,
        entities::DbSchema
    },
    traits::SearchServiceStorage
};


pub async fn get_filter_properties(
    db_schema: &DbSchema,
    table_search: &TableSearch,
    storage: &Arc<dyn SearchServiceStorage>
) -> Result<Properties,Error> {

    let properties = Properties::new(
        db_schema.clone(),
        table_search.clone(),
        storage
    );

    Ok(properties)
}

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
    pub fn new(
        db_schema: DbSchema,
        table_search: TableSearch,
        storage: &Arc<dyn SearchServiceStorage>
    ) -> Self {

        let mut attributes : Vec<AttributeInfo> = Vec::new();
        let mut attributes_subsets : Vec<HashSet<u8>> = Vec::new();
        let mut tables_subsets : Vec<HashSet<String>> = Vec::new();

        for table in db_schema.tables {
            let full_table_name = format!("{}.{}",&table.schema,&table.name).to_string();

            let table_subset_id = manage_subsets(&full_table_name, &mut tables_subsets,
                &mut attributes_subsets, &table_search).expect("Error finding table subset id");
            for attribute in table.attributes{
                let full_attr_name = format!("{}.{}",full_table_name,&attribute.name).to_string();

                let data_type = storage
                    .translate_native_type(&attribute.data_type)
                    .expect(&format!("Error translating data type: {}",attribute.data_type));

                let attribute_info = AttributeInfo::new(
                    full_attr_name,
                    data_type,
                    table_subset_id
                );
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

    use crate::storage::postgres::{PostgresConfig,PostgresStorage};

    use crate::storage::mysql::{MySQLConfig,MySQLStorage};

    use crate::relational::table_search::entities::TableSearchInfo;

    async fn aux_get_db_schema(storage: &Arc<dyn SearchServiceStorage>) -> Result<DbSchema,Error> {
        let db_schema_info = storage.get_db_schema_info().await?;
        Ok(db_schema_info)
    }

    async fn aux_get_pg_storage() -> Result< Arc<dyn SearchServiceStorage>, Error> {
        let storage: Arc<dyn SearchServiceStorage> = Arc::new(
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

    async fn aux_get_mysql_storage() -> Result< Arc<dyn SearchServiceStorage>, Error> {
        let storage: Arc<dyn SearchServiceStorage> = Arc::new(
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

    fn aux_get_table_search(db_schema: &DbSchema) -> Result<TableSearch,Error> {

        let tables_search_info: Vec<TableSearchInfo> = db_schema.tables
            .clone()
            .into_iter()
            .map(TableSearchInfo::from)
            .collect();

        let table_search = TableSearch::new(tables_search_info, db_schema.foreign_keys.clone());
        
        Ok(table_search)
    }

    #[tokio::test]
    async fn test_operators_creation_pg() -> Result<(), Error> {

        let db_storage = aux_get_pg_storage().await?;
        let db_schema = aux_get_db_schema(&db_storage).await?;
        let table_search = aux_get_table_search(&db_schema)?;
        let properties = Properties::new(db_schema, table_search, &db_storage);

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
    async fn test_logical_operators_creation_mysql() -> Result<(), Error> {

        let db_storage = aux_get_mysql_storage().await?;
        let db_schema = aux_get_db_schema(&db_storage).await?;
        let table_search = aux_get_table_search(&db_schema)?;
        let properties = Properties::new(db_schema, table_search, &db_storage);

        assert_eq!(properties.logical_operators,
            vec![
                "AND".to_string(),
                "OR".to_string(),
            ]
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_attributes_types_correctness_pg() -> Result<(), Error> {

        let db_storage = aux_get_mysql_storage().await?;
        let db_schema = aux_get_db_schema(&db_storage).await?;
        let table_search = aux_get_table_search(&db_schema)?;
        let properties = Properties::new(db_schema, table_search, &db_storage);

        let data_type_vec = DataType::iter().collect::<Vec<_>>();
        let possible_data_types : Vec<String> = data_type_vec
            .iter()
            .map(|o| o.clone().to_string())
            .collect();

        for attribute in properties.attributes {
            assert_eq!(
                possible_data_types.contains(&attribute.data_type.to_string()),
                true
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_subsets_correctness_mysql() -> Result<(), Error> {

        let db_storage = aux_get_mysql_storage().await?;
        let db_schema = aux_get_db_schema(&db_storage).await?;
        let table_search = aux_get_table_search(&db_schema)?;
        let properties = Properties::new(db_schema, table_search, &db_storage);

        for attribute in properties.attributes {
            assert_eq!(attribute.subset_id,0);
        }

        assert_eq!(
            properties.subsets.len(),
            1,
        );


        Ok(())

    }

}