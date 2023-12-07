pub mod properties;

use std::sync::Arc;

use thiserror::Error;

use crate::{
    manager::properties::{Properties, PropertiesManager},
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
    pub properties_manager: PropertiesManager,
    pub storage: Arc<dyn SearchServiceStorage>,
}

impl SearchServiceManager {
    pub async fn new(storage: Arc<dyn SearchServiceStorage>) -> Self {
        Self {
            storage: storage.clone(),
            properties_manager: PropertiesManager::new(storage),
        }
    }

    pub async fn get_filter_properties(&self) -> Result<Properties, ManagerError> {
        let db_schema = self.storage.get_db_schema_info().await?;
        let table_search = self.get_table_search(&db_schema).await?;
        Ok(self
            .properties_manager
            .get_filter_properties(&db_schema, &table_search)
            .await?)
    }

    pub async fn search(
        &self,
        projection: Vec<String>,
        filters: String,
    ) -> Result<serde_json::Value, ManagerError> {
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

        let table_search = self
            .get_table_search(&self.storage.get_db_schema_info().await?)
            .await?;

        let visitor = DatabaseVisitor::new(table_search);

        let query = command
            .accept(projection, Arc::new(visitor))
            .map_err(|e| ManagerError::QueryBuildError(e.to_string()))?;

        let res = self.storage.execute(query).await?;

        let res = serde_json::json!({
            "search_result": serde_json::json!(res),
        });
        Ok(res)
    }

    async fn get_table_search(&self, db_schema: &DbSchema) -> Result<TableSearch, ManagerError> {
        let tables_search_info: Vec<TableSearchInfo> = db_schema
            .tables
            .clone()
            .into_iter()
            .map(TableSearchInfo::from)
            .collect();

        let table_search = TableSearch::new(tables_search_info, db_schema.foreign_keys.clone());

        Ok(table_search)
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    use anyhow::Error;

    use serde_json::json;

    use crate::storage::postgres::{PostgresConfig, PostgresStorage};

    use crate::storage::mysql::{MySQLConfig, MySQLStorage};

    async fn aux_get_pg_storage() -> Result<Arc<dyn SearchServiceStorage>, Error> {
        let storage: Arc<dyn SearchServiceStorage> = Arc::new(
            PostgresStorage::new(PostgresConfig::new(
                "public,movies".into(),
                "localhost".into(),
                54329,
                "search-service".into(),
                "search-service".into(),
                "search-service".into(),
            ))
            .await?,
        );
        Ok(storage)
    }

    async fn aux_get_mysql_storage() -> Result<Arc<dyn SearchServiceStorage>, Error> {
        let storage: Arc<dyn SearchServiceStorage> = Arc::new(
            MySQLStorage::new(MySQLConfig::new(
                "public,movies".into(),
                "localhost".into(),
                3306,
                "searchservice".into(),
                "searchservice".into(),
                "searchservice".into(),
            ))
            .await?,
        );
        Ok(storage)
    }

    #[tokio::test]
    async fn test_search_service_pg() -> Result<(), Error> {
        let db_storage = aux_get_pg_storage().await?;

        let projection = vec![
            "movies.person.person_name::TEXT".to_string(),
            "movies.movie_cast.character_name::TEXT".to_string(),
            "movies.movie.title::TEXT".to_string()
        ];
        let filters = "movies.movie_cast.character_name eq Harry Potter".to_string();

        let search_manager = SearchServiceManager::new(db_storage).await;

        let search_result = search_manager.search(projection,filters).await?;

        assert_ne!(search_result["search_result"],json!([]));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_service_mysql() -> Result<(), Error> {
        let db_storage = aux_get_mysql_storage().await?;

        let projection = vec![
            "movies.movie.title".to_string()
        ];
        let filters = "movies.person.person_name eq Wagner Moura".to_string();

        let search_manager = SearchServiceManager::new(db_storage).await;

        let search_result = search_manager.search(projection,filters).await?;

        assert_ne!(search_result["search_result"],json!([]));

        Ok(())
    }

}

