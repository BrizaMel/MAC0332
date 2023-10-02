#[cfg(test)]
mod tests {

    use std::env;
    use crate::postgres::{PostgresConfig,PostgresStorage};
    use deadpool_postgres::Object;
       
    // use super::*;

    fn setup_env(){
        env::set_var("ALLOWED_SCHEMAS", "public,movies");
        env::set_var("DB_HOST", "localhost");
        env::set_var("DB_PORT", "54329");
        env::set_var("DB_USER", "search-service");
        env::set_var("DB_PASS", "search-service");
        env::set_var("DB_NAME", "search-service");        
    }

    async fn setup_storage() -> PostgresStorage{
        setup_env();
        let storage = PostgresStorage::new(PostgresConfig::new()).await.unwrap();
        storage
    }

    async fn setup_client() -> Object{
        let storage = setup_storage().await;
        let client = storage.get_client().await.unwrap();
        client
    }

    #[tokio::test]
    async fn test_get_client(){
        let client = setup_client().await;
        let x = 10;
        let stmt = client.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&x]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, 11);
    }

    #[tokio::test]
    async fn test_get_db_tables(){
        let storage = setup_storage().await;
        let client = setup_client().await;

        let tables = storage.get_db_tables(&client,&storage.allowed_schemas).await.unwrap();
        
        assert_eq!(tables.len(), 17);
        assert!(tables.iter().any(|t| t.name == "movie"));
        assert!(tables.iter().any(|t| t.name == "production_country"));
    }

    #[tokio::test]
    async fn test_get_table_attributes(){
        let storage = setup_storage().await;
        let client = setup_client().await;

        let table_attributes = storage.get_table_attributes(&"movies".to_string(),&"movie".to_string(),&client).await.unwrap();
        assert_eq!(table_attributes.len(), 13);
        assert!(table_attributes.iter().any(|a| a.name == "title"));
        assert!(table_attributes.iter().any(|a| a.name == "release_date"));
    }

    #[tokio::test]
    async fn test_get_table_primary_keys(){
        let storage = setup_storage().await;
        let client = setup_client().await;

        let table_primary_keys = storage.get_table_primary_keys(&"movies".to_string(),&"person".to_string(),&client).await.unwrap();
        assert_eq!(table_primary_keys.len(), 1);
        assert!(table_primary_keys.iter().any(|pk| pk.attribute_name == "person_id"));
        let table_zero_primary_keys = storage.get_table_primary_keys(&"movies".to_string(),&"production_country".to_string(),&client).await.unwrap();
        assert_eq!(table_zero_primary_keys.len(), 0);
    }

    #[tokio::test]
    async fn test_get_db_foreign_keys(){
        let storage = setup_storage().await;
        let client = setup_client().await;

        let foreign_keys = storage.get_db_foreign_keys(&client,&storage.allowed_schemas).await.unwrap();
        assert_eq!(foreign_keys.len(), 17);
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "language_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "movie_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "language_role_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_genres" &&  pk.table_name_foreign == "genre"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_genres" &&  pk.table_name_foreign == "movie"));
    }

    #[tokio::test]
    async fn test_get_db_schema_info(){
        let storage = setup_storage().await;

        let schema_info = storage.get_db_schema_info().await.unwrap();
        assert!(schema_info.tables.len() > 0);
        assert!(schema_info.foreign_keys.len() > 0);
    }

}