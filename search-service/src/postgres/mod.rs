use anyhow::{Ok, Result, Error};
use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub mod queries;
pub mod tests;

use crate::relational::entities::{Attribute, DbSchema, ForeignKey, PrimaryKey, Table};

pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub allowed_schemas: Vec<String>,
}

impl PostgresConfig {
    pub fn new(allowed_schemas_string: String, db_host : String, db_port: u16, postgres_user: String,postgres_pass: String, postgres_db: String) -> Self {
        let allowed_schemas: Vec<String> = allowed_schemas_string
            .split(",")
            .map(|s| s.to_string())
            .collect();        
        Self {
            host: db_host,
            port: db_port,
            user: postgres_user,
            password: postgres_pass,
            dbname: postgres_db,
            allowed_schemas: allowed_schemas,
        }
    }

    pub fn from_env() -> Self {
        let allowed_schemas_var =
            std::env::var("ALLOWED_SCHEMAS").unwrap_or_else(|_| "public".to_string());
        let allowed_schemas: Vec<String> = allowed_schemas_var
            .split(",")
            .map(|s| s.to_string())
            .collect();
        Self {
            host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("DB_PORT")
                .unwrap_or_else(|_| "54329".to_string())
                .parse::<u16>()
                .unwrap(),
            user: std::env::var("DB_USER").unwrap_or_else(|_| "search-service".to_string()),
            password: std::env::var("DB_PASS").unwrap_or_else(|_| "search-service".to_string()),
            dbname: std::env::var("DB_NAME").unwrap_or_else(|_| "search-service".to_string()),
            allowed_schemas: allowed_schemas,
        }
    }
}

pub struct PostgresStorage {
    pub pool: Pool,
    pub allowed_schemas: Vec<String>,
}

impl PostgresStorage {
    pub async fn new(config: PostgresConfig) -> Result<Self> {
        let mut pg_config = tokio_postgres::Config::new();

        pg_config.user(&config.user);
        pg_config.password(&config.password);
        pg_config.dbname(&config.dbname);
        pg_config.host(&config.host);
        pg_config.port(config.port);
        pg_config.application_name("search-service");

        println!("Database host: {}", config.host);

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

        let pool = Pool::builder(mgr).build()?;

        let allowed_schemas = config.allowed_schemas;

        println!("Allowed Schemas: {:?}", allowed_schemas);

        Ok(Self {
            pool,
            allowed_schemas,
        })
    }

    async fn get_client(&self) -> Result<Object> {
        let client = self.pool.get().await?;

        Ok(client)
    }

    pub async fn get_db_schema_info(&self) -> Result<DbSchema,Error> {
        let allowed_schemas: &Vec<String> = &self.allowed_schemas;

        let this_client = self
            .get_client()
            .await
            .expect("Unable to retrieve Postgres Client");
        let tables = self
            .get_db_tables(&this_client, &allowed_schemas)
            .await
            .expect("Error retireving Database Tables");
        let foreign_keys = self
            .get_db_foreign_keys(&this_client, &allowed_schemas)
            .await
            .expect("Error retireving Database Foreign Keys");

        let db_schema: DbSchema = DbSchema::new(tables, foreign_keys);
        Ok(db_schema)
    }

    async fn get_db_tables(
        &self,
        client: &Object,
        allowed_schemas: &Vec<String>,
    ) -> Result<Vec<Table>> {
        let mut table_vec: Vec<Table> = Vec::new();

        // Search for tables
        for tables_row in client
            .query(queries::GET_TABLES, &[&allowed_schemas])
            .await
            .expect("Error retrieving tables")
        {
            let table_schema: String = tables_row.get("table_schema");
            let table_name: String = tables_row.get("table_name");

            let attributes_vec: Vec<Attribute> = self
                .get_table_attributes(&table_schema, &table_name, &client)
                .await
                .expect("Error retrieving attributes");

            let primary_keys_vec: Vec<PrimaryKey> = self
                .get_table_primary_keys(&table_schema, &table_name, &client)
                .await
                .expect("Error retrieving primary keys");

            let table: Table =
                Table::new(table_schema, table_name, attributes_vec, primary_keys_vec);
            table_vec.push(table);
        }

        Ok(table_vec)
    }

    async fn get_table_attributes(
        &self,
        table_schema: &String,
        table_name: &String,
        client: &Object,
    ) -> Result<Vec<Attribute>, anyhow::Error> {
        let mut attributes_vec: Vec<Attribute> = Vec::new();

        // For each table, search for its attributes
        for attributes_row in client
            .query(queries::GET_ATTRIBUTES, &[&table_schema, &table_name])
            .await
            .expect("Error retrieving attributes")
        {
            let attribute: Attribute = Attribute::new(
                attributes_row.try_get("column_name")?,
                attributes_row.try_get("data_type")?,
            );

            attributes_vec.push(attribute);
        }

        Ok(attributes_vec)
    }

    async fn get_table_primary_keys(
        &self,
        table_schema: &String,
        table_name: &String,
        client: &Object,
    ) -> Result<Vec<PrimaryKey>, anyhow::Error> {
        let mut primary_keys_vec: Vec<PrimaryKey> = Vec::new();

        // For each table, search for its primary_keys
        for primary_keys_row in client
            .query(queries::GET_PRIMARY_KEYS, &[&table_schema, &table_name])
            .await
            .expect("Error retrieving primary keys")
        {
            let primary_key: PrimaryKey = PrimaryKey::new(
                primary_keys_row.try_get("table_schema")?,
                primary_keys_row.try_get("table_name")?,
                primary_keys_row.try_get("column_name")?,
            );

            primary_keys_vec.push(primary_key);
        }

        Ok(primary_keys_vec)
    }

    async fn get_db_foreign_keys(
        &self,
        client: &Object,
        allowed_schemas: &Vec<String>,
    ) -> Result<Vec<ForeignKey>, anyhow::Error> {
        let mut foreign_keys_vec: Vec<ForeignKey> = Vec::new();
        let query_error: &str = "Error retrieving foreign keys";

        // Search for foreign keys
        for foreign_keys_rows in client
            .query(queries::GET_FOREIGN_KEYS, &[&allowed_schemas])
            .await
            .expect(query_error)
        {
            let foreign_key: ForeignKey = ForeignKey::new(
                foreign_keys_rows.try_get("table_schema")?,
                foreign_keys_rows.try_get("table_name")?,
                foreign_keys_rows.try_get("column_name")?,
                foreign_keys_rows.try_get("foreign_table_schema")?,
                foreign_keys_rows.try_get("foreign_table_name")?,
                foreign_keys_rows.try_get("foreign_column_name")?,
            );

            foreign_keys_vec.push(foreign_key);
        }

        Ok(foreign_keys_vec)
    }

}
