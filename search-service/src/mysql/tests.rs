#[cfg(test)]
mod tests {

    use mysql::{PooledConn};

    use mysql::prelude::Queryable;

    use crate::mysql::{MySQLConfig, MySQLStorage};

    use anyhow::Error;

    async fn setup_storage() -> MySQLStorage {

        let storage = MySQLStorage::new(
            MySQLConfig::new(
                "public,movies".into(),
                "localhost".into(),
                3306,
                "searchservice".into(),
                "searchservice".into(),
                "searchservice".into()
            )
        ).await.unwrap();

        storage
    }

    async fn setup_client() -> PooledConn {
        let storage = setup_storage().await;
        let client = storage.get_client().unwrap();
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

}
