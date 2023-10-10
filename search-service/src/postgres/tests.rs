#[cfg(test)]
mod tests {

    use std::env;

    use crate::postgres::{PostgresConfig,PostgresStorage};

    use anyhow::Error;
    
    use deadpool_postgres::Object;
    
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
    async fn test_get_client() -> Result<(),Error>{
        let client = setup_client().await;
        let expected_result = 11;
        let rows = client.query("SELECT 1 + 10", &[]).await?;
        let value: i32 = rows[0].get(0);
        
        assert_eq!(value, expected_result);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_tables() -> Result<(),Error> {
        let storage = setup_storage().await;
        let client = setup_client().await;

        let tables = storage.get_db_tables(&client,&storage.allowed_schemas).await?;
        let expected_table_qty = 17;

        assert_eq!(tables.len(), expected_table_qty);
        assert!(tables.iter().any(|t| t.name == "movie"));
        assert!(tables.iter().any(|t| t.name == "production_country"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_attributes()  -> Result<(),Error>{
        let storage = setup_storage().await;
        let client = setup_client().await;

        let table_attributes = storage.get_table_attributes(&"movies".to_string(),&"movie".to_string(),&client).await?;
        let expected_attribute_qty = 13;

        assert_eq!(table_attributes.len(), expected_attribute_qty);
        assert!(table_attributes.iter().any(|a| a.name == "title"));
        assert!(table_attributes.iter().any(|a| a.name == "release_date"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_primary_keys()  -> Result<(),Error>{
        let storage = setup_storage().await;
        let client = setup_client().await;

        let table_primary_keys = storage.get_table_primary_keys(&"movies".to_string(),&"person".to_string(),&client).await?;
        let expected_pkeys_qty = 1;
        
        assert_eq!(table_primary_keys.len(), expected_pkeys_qty);
        assert!(table_primary_keys.iter().any(|pk| pk.attribute_name == "person_id"));
        
        let table_zero_primary_keys = storage.get_table_primary_keys(&"movies".to_string(),&"production_country".to_string(),&client).await?;
        let expected_zero_pkeys_qty = 0;

        assert_eq!(table_zero_primary_keys.len(), expected_zero_pkeys_qty);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_foreign_keys() -> Result<(),Error>{
        let storage = setup_storage().await;
        let client = setup_client().await;

        let foreign_keys = storage.get_db_foreign_keys(&client,&storage.allowed_schemas).await?;
        let expected_fkeys_qty = 17;

        assert_eq!(foreign_keys.len(), expected_fkeys_qty);
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "language_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "movie_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_languages" &&  pk.attribute_name == "language_role_id"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_genres" &&  pk.table_name_foreign == "genre"));
        assert!(foreign_keys.iter().any(|pk| pk.table_name == "movie_genres" &&  pk.table_name_foreign == "movie"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_schema_info()  -> Result<(),Error>{
        let storage = setup_storage().await;

        let schema_info = storage.get_db_schema_info().await?;
        
        assert!(schema_info.tables.len() > 0);
        assert!(schema_info.foreign_keys.len() > 0);

        Ok(())
    }

}