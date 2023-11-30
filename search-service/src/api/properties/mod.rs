use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

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
        
        for table in schema_info.tables {
            for attribute in table.attributes{
                let mut full_attr_name = "".to_string();
                full_attr_name.push_str(&table.schema);  
                full_attr_name.push_str(".");  
                full_attr_name.push_str(&table.name);  
                full_attr_name.push_str(".");  
                full_attr_name.push_str(&attribute.name);

                let data_type = db_storage
                    .translate_native_type(&attribute.data_type)
                    .expect(&format!("Error translating data type: {}",attribute.data_type));

                let subset_id = 0;
                
                let attribute_info = AttributeInfo::new(full_attr_name,data_type,subset_id);
                attributes.push(attribute_info);
            }       
        }

        let subsets : Vec<HashSet<u8>> = Vec::new();

        Self {
            attributes,
            subsets,
            operators
        }
    }

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
}

