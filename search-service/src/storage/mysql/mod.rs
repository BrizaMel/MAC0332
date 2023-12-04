use anyhow::{Error, Ok, Result};
use async_trait::async_trait;
use mysql::prelude::Queryable;
use mysql::{from_row, params, OptsBuilder, Pool, PooledConn};
use std::time::Duration;

use crate::relational::entities::{Attribute, DbSchema, ForeignKey, PrimaryKey, Table};
use crate::traits::SearchServiceStorage;

use self::utils::row_to_json;

pub mod queries;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub allowed_schemas: Vec<String>,
}

impl MySQLConfig {
    pub fn new(
        allowed_schemas_string: String,
        db_host: String,
        db_port: u16,
        mysql_user: String,
        mysql_pass: String,
        mysql_db: String,
    ) -> Self {
        let allowed_schemas: Vec<String> = allowed_schemas_string
            .split(",")
            .map(|s| s.to_string())
            .collect();
        Self {
            host: db_host,
            port: db_port,
            user: mysql_user,
            password: mysql_pass,
            dbname: mysql_db,
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
                .unwrap_or_else(|_| "3306".to_string())
                .parse::<u16>()
                .unwrap(),
            user: std::env::var("DB_USER").unwrap_or_else(|_| "searchservice".to_string()),
            password: std::env::var("DB_PASS").unwrap_or_else(|_| "searchservice".to_string()),
            dbname: std::env::var("DB_NAME").unwrap_or_else(|_| "searchservice".to_string()),
            allowed_schemas: allowed_schemas,
        }
    }
}

pub struct MySQLStorage {
    pub pool: Pool,
    pub allowed_schemas: Vec<String>,
}

impl MySQLStorage {
    pub async fn new(config: MySQLConfig) -> Result<Self, Error> {
        let mysql_opts = OptsBuilder::new()
            .user(Some(config.user.to_owned()))
            .pass(Some(config.password.to_owned()))
            .ip_or_hostname(Some(config.host.to_owned()))
            .tcp_port(config.port.to_owned())
            .db_name(Some(config.dbname.to_owned()))
            .tcp_connect_timeout(Some(Duration::new(10, 0)))
            .read_timeout(Some(Duration::new(3, 0)));

        let pool = Pool::new(mysql_opts)?;

        let allowed_schemas = config.allowed_schemas;

        println!("Allowed Schemas (MySQL): {:?}", allowed_schemas);

        Ok(Self {
            pool,
            allowed_schemas,
        })
    }

    fn get_client(&self) -> Result<PooledConn, Error> {
        let client = self.pool.get_conn()?;

        Ok(client)
    }

    pub async fn get_db_schema_info(&self) -> Result<DbSchema, Error> {
        let allowed_schemas: &Vec<String> = &self.allowed_schemas;

        let tables = self
            .get_db_tables(&allowed_schemas)
            .await
            .expect("Error retireving Database Tables");
        let foreign_keys = self
            .get_db_foreign_keys(&allowed_schemas)
            .await
            .expect("Error retireving Database Foreign Keys");

        let db_schema: DbSchema = DbSchema::new(tables, foreign_keys);
        Ok(db_schema)
    }

    async fn get_db_tables(&self, allowed_schemas: &Vec<String>) -> Result<Vec<Table>, Error> {
        let mut client = self.get_client()?;

        let mut table_vec: Vec<Table> = Vec::new();

        let params = vec_to_mysql_list(&allowed_schemas)?;

        let query_str: String = queries::GET_TABLES.replace(":allowed_schemas", params.as_str());

        for tables_row in client.query_iter(query_str)? {
            let (table_schema, table_name): (String, String) = from_row(tables_row?);

            let attributes_vec: Vec<Attribute> = self
                .get_table_attributes(&table_schema, &table_name)
                .await
                .expect("Error retrieving attributes");

            let primary_keys_vec: Vec<PrimaryKey> = self
                .get_table_primary_keys(&table_schema, &table_name)
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
    ) -> Result<Vec<Attribute>, Error> {
        let mut client = self.get_client()?;

        let mut attributes_vec: Vec<Attribute> = Vec::new();

        // For each table, search for its attributes
        for attributes_row in client.exec_iter(
            queries::GET_ATTRIBUTES,
            params! {
                "table_schema" => table_schema,
                "table_name" => table_name
            },
        )? {
            let (column_name, data_type): (String, String) = from_row(attributes_row?);

            let attribute: Attribute = Attribute::new(column_name, data_type);

            attributes_vec.push(attribute);
        }

        Ok(attributes_vec)
    }

    async fn get_table_primary_keys(
        &self,
        table_schema: &String,
        table_name: &String,
    ) -> Result<Vec<PrimaryKey>, Error> {
        let mut client = self.get_client()?;

        let mut primary_keys_vec: Vec<PrimaryKey> = Vec::new();

        // For each table, search for its primary_keys
        for primary_keys_row in client.exec_iter(
            queries::GET_PRIMARY_KEYS,
            params! {
                "table_schema" => table_schema,
                "table_name" => table_name
            },
        )? {
            let column_name: String = from_row(primary_keys_row?);

            let primary_key: PrimaryKey = PrimaryKey::new(
                table_schema.to_string(),
                table_name.to_string(),
                column_name.to_string(),
            );

            primary_keys_vec.push(primary_key);
        }

        Ok(primary_keys_vec)
    }

    async fn get_db_foreign_keys(
        &self,
        allowed_schemas: &Vec<String>,
    ) -> Result<Vec<ForeignKey>, Error> {
        let mut client = self.get_client()?;

        let mut foreign_keys_vec: Vec<ForeignKey> = Vec::new();

        let params = vec_to_mysql_list(&allowed_schemas)?;

        let query_str: String =
            queries::GET_FOREIGN_KEYS.replace(":allowed_schemas", params.as_str());

        // Search for foreign keys
        for foreign_keys_rows in client.query_iter(query_str)? {
            let (
                table_schema,
                table_name,
                column_name,
                foreign_table_schema,
                foreign_table_name,
                foreign_column_name,
            ): (String, String, String, String, String, String) = from_row(foreign_keys_rows?);

            let foreign_key: ForeignKey = ForeignKey::new(
                table_schema,
                table_name,
                column_name,
                foreign_table_schema,
                foreign_table_name,
                foreign_column_name,
            );

            foreign_keys_vec.push(foreign_key);
        }

        Ok(foreign_keys_vec)
    }
}

#[async_trait]
impl SearchServiceStorage for MySQLStorage {
    async fn get_db_schema_info(&self) -> Result<DbSchema, Error> {
        let allowed_schemas: &Vec<String> = &self.allowed_schemas;

        let tables = self
            .get_db_tables(&allowed_schemas)
            .await
            .expect("Error retireving Database Tables");
        let foreign_keys = self
            .get_db_foreign_keys(&allowed_schemas)
            .await
            .expect("Error retireving Database Foreign Keys");

        let db_schema: DbSchema = DbSchema::new(tables, foreign_keys);
        Ok(db_schema)
    }

    async fn execute(&self, query: String) -> Result<Vec<serde_json::Value>, Error> {
        let mut conn = self.get_client()?;

        let rows = conn.query_iter(query)?;

        Ok(rows
            .into_iter()
            .map(|row| row_to_json(row?))
            .collect::<Result<Vec<serde_json::Value>>>()?)
    }

    fn get_database(&self) -> &str {
        "mysql"
    }
}

fn vec_to_mysql_list(v: &Vec<String>) -> Result<String> {
    let mut params = "".to_string();
    for (idx, item) in v.iter().enumerate() {
        params.push_str("'");
        params.push_str(item);
        params.push_str("'");
        if idx != v.len() - 1 {
            params.push_str(", ");
        }
    }
    Ok(params)
}
