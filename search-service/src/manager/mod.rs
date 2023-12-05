pub mod properties;

use std::sync::Arc;

use thiserror::Error;

use crate::{
    manager::properties::{
        get_filter_properties,
        Properties
    },
    query_representation::initial::initial_to_command,
    relational::{
        entities::DbSchema,
        table_search::{entities::TableSearchInfo, TableSearch},
    },
    storage::DatabaseVisitor,
    traits::{Component, SearchServiceStorage},
};

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error("Failed to parse initial query representation: {0}")]
    ParseError(String),

    #[error("Failed to build query: {0}")]
    QueryBuildError(String),
}

#[derive(Clone)]
pub struct SearchServiceManager {
    pub storage: Arc<dyn SearchServiceStorage>,
}

impl SearchServiceManager {
    pub async fn new(storage: Arc<dyn SearchServiceStorage>) -> Self {
        Self { storage }
    }

    pub async fn get_filter_properties(&self) -> Result<Properties, ManagerError> {
        let db_schema = self.storage.get_db_schema_info().await?;
        let table_search = self.get_table_search(&db_schema).await?;
        Ok(get_filter_properties(&db_schema, &table_search, &self.storage).await?)
    }

    pub async fn search(
        &self,
        projection: Vec<String>,
        filters: String,
    ) -> Result<Vec<serde_json::Value>, ManagerError> {
        let projection = match self.storage.get_database() {
            "postgres" => projection
                .iter()
                .map(|att| format!("{}::TEXT", att))
                .collect(),
            "mysql" => projection,
            _ => projection,
        };

        let command =
            initial_to_command(filters).map_err(|e| ManagerError::ParseError(e.to_string()))?;

        let table_search = self.get_table_search(
            &self.storage.get_db_schema_info().await?
        ).await?;

        let visitor = DatabaseVisitor::new(table_search);

        let query = command
            .accept(projection, Arc::new(visitor))
            .map_err(|e| ManagerError::QueryBuildError(e.to_string()))?;

        Ok(self.storage.execute(query).await?)
    }

    async fn get_table_search(&self, db_schema: &DbSchema) -> Result<TableSearch, ManagerError> {
        let tables_search_info: Vec<TableSearchInfo> = db_schema.tables
            .clone()
            .into_iter()
            .map(TableSearchInfo::from)
            .collect();
            
        let table_search = TableSearch::new(tables_search_info, db_schema.foreign_keys.clone());

        Ok(table_search)
    }
}
