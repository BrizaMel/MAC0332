#[cfg(test)]
mod tests {
    use mysql::PooledConn;

    use mysql::prelude::Queryable;

    use anyhow::Error;

    use crate::storage::mysql::{MySQLConfig, MySQLStorage};

    use crate::traits::SearchServiceStorage;

    use crate::query_representation::intermediary::single_command::DataType;

    async fn setup_storage() -> MySQLStorage {
        let storage = MySQLStorage::new(MySQLConfig::new(
            "public,movies".into(),
            "localhost".into(),
            3306,
            "searchservice".into(),
            "searchservice".into(),
            "searchservice".into(),
        ))
        .await
        .expect("Error initializing MySQLStorage");

        storage
    }

    async fn setup_client() -> PooledConn {
        let storage = setup_storage().await;
        let client = storage.get_client().expect("Error getting MySQL client");
        client
    }

    #[tokio::test]
    async fn test_get_client() -> Result<(), Error> {
        let mut client = setup_client().await;
        let expected_result = 11;
        let result: u8 = client.query_first("SELECT 1 + 10").unwrap().unwrap();

        assert_eq!(result, expected_result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_tables() -> Result<(), Error> {
        let storage = setup_storage().await;

        let tables = storage.get_db_tables(&storage.allowed_schemas).await?;
        let expected_table_qty = 17;

        assert_eq!(tables.len(), expected_table_qty);
        assert!(tables.iter().any(|t| t.name == "movie"));
        assert!(tables.iter().any(|t| t.name == "production_country"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_attributes() -> Result<(), Error> {
        let storage = setup_storage().await;

        let table_attributes = storage
            .get_table_attributes(&"movies".to_string(), &"movie".to_string())
            .await?;

        let expected_attribute_qty = 13;

        assert_eq!(table_attributes.len(), expected_attribute_qty);
        assert!(table_attributes.iter().any(|a| a.name == "title"));
        assert!(table_attributes.iter().any(|a| a.name == "release_date"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_primary_keys() -> Result<(), Error> {
        let storage = setup_storage().await;

        let table_primary_keys = storage
            .get_table_primary_keys(&"movies".to_string(), &"person".to_string())
            .await?;
        let expected_pkeys_qty = 1;

        assert_eq!(table_primary_keys.len(), expected_pkeys_qty);
        assert!(table_primary_keys
            .iter()
            .any(|pk| pk.attribute_name == "person_id"));

        let table_zero_primary_keys = storage
            .get_table_primary_keys(&"movies".to_string(), &"production_country".to_string())
            .await?;
        let expected_zero_pkeys_qty = 0;

        assert_eq!(table_zero_primary_keys.len(), expected_zero_pkeys_qty);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_foreign_keys() -> Result<(), Error> {
        let storage = setup_storage().await;

        let foreign_keys = storage
            .get_db_foreign_keys(&storage.allowed_schemas)
            .await?;
        let expected_fkeys_qty = 17;

        assert_eq!(foreign_keys.len(), expected_fkeys_qty);
        assert!(foreign_keys
            .iter()
            .any(|pk| pk.table_name == "movie_languages" && pk.attribute_name == "language_id"));
        assert!(foreign_keys
            .iter()
            .any(|pk| pk.table_name == "movie_languages" && pk.attribute_name == "movie_id"));
        assert!(foreign_keys.iter().any(
            |pk| pk.table_name == "movie_languages" && pk.attribute_name == "language_role_id"
        ));
        assert!(foreign_keys
            .iter()
            .any(|pk| pk.table_name == "movie_genres" && pk.table_name_foreign == "genre"));
        assert!(foreign_keys
            .iter()
            .any(|pk| pk.table_name == "movie_genres" && pk.table_name_foreign == "movie"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_db_schema_info() -> Result<(), Error> {
        let storage = setup_storage().await;
        let schema_info = storage.get_db_schema_info().await?;

        assert!(schema_info.tables.len() > 0);
        assert!(schema_info.foreign_keys.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let storage = setup_storage().await;

        let json = storage.execute(
            "SELECT movies.movie_cast.character_name,movies.person.person_name \
            FROM movies.movie_cast, movies.person \
            WHERE movies.movie_cast.person_id = movies.person.person_id \
            ORDER BY movies.person.person_name ASC \
            LIMIT 4;".to_string()).await?;
        assert_eq!(json.len(),4);
        assert_eq!(json[0]["character_name"],"El Chiquis".to_string());
        assert_eq!(json[3]["person_name"],"'Snub' Pollard".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_translate_native_type() -> Result<(), Error> {

        let storage = setup_storage().await;

        let mut native_type = "int".into();
        assert_eq!(storage.translate_native_type(native_type)?,DataType::Integer);

        native_type = "varchar".into();
        assert_eq!(storage.translate_native_type(native_type)?,DataType::String);

        native_type = "decimal".into();
        assert_eq!(storage.translate_native_type(native_type)?,DataType::Float);

        native_type = "bigint".into();
        assert_eq!(storage.translate_native_type(native_type)?,DataType::Integer);

        native_type = "date".into();
        assert_eq!(storage.translate_native_type(native_type)?,DataType::Date);
        Ok(())
    }

}
