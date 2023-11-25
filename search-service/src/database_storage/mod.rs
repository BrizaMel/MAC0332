use anyhow::Error;

use async_trait::async_trait;

pub mod visitor;

use crate::traits::DatabaseOperations;

use crate::postgres::{PostgresStorage,PostgresConfig};

use crate::mysql::{MySQLStorage,MySQLConfig};

use crate::relational::entities::DbSchema;

pub enum DatabaseStorage {
	PostgresStorage(PostgresStorage),
	MySQLStorage(MySQLStorage)
}

#[async_trait]
impl DatabaseOperations for DatabaseStorage {
	async fn get_db_schema_info(&self) -> Result<DbSchema,Error> {
		let db_schema = match &self {
			DatabaseStorage::PostgresStorage(ps) => 
				ps
				.get_db_schema_info()
				.await
				.expect("Error retireving Database Schema Information (Postgres)"),
			DatabaseStorage::MySQLStorage(ms) => 
				ms
				.get_db_schema_info()
				.await
				.expect("Error retireving Database Schema Information (MySQL)")				
		};
		Ok(db_schema)
	}
}

impl DatabaseStorage {
	pub async fn new() -> Self {
		let dbsm_to_connect = std::env::var("DMBS").unwrap_or_else(|_| panic!("DBMS variable missing in the environment"));

		let chosen_storage = match dbsm_to_connect.as_str() {
			"postgres" => DatabaseStorage::PostgresStorage(
				PostgresStorage::new(
					PostgresConfig::from_env()
				)
				.await
				.expect("Error initializing psql_storage")),

			"mysql" => DatabaseStorage::MySQLStorage(
				MySQLStorage::new(
					MySQLConfig::from_env()
				)
				.await
				.expect("Error initializing mysql_storage")),

			_ => panic!("Invalid DBMS in the environment")
		};

		chosen_storage
	}
}