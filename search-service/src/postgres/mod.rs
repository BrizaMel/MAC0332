use anyhow::{Ok, Result};
use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use serde::{Deserialize, Serialize};
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl PostgresConfig {
    pub fn new() -> Self {
        Self {
            host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("DB_PORT")
                .unwrap_or_else(|_| "54329".to_string())
                .parse::<u16>()
                .unwrap(),
            user: std::env::var("DB_USER").unwrap_or_else(|_| "search-service".to_string()),
            password: std::env::var("DB_PASS").unwrap_or_else(|_| "search-service".to_string()),
            dbname: std::env::var("DB_NAME").unwrap_or_else(|_| "search-service".to_string()),
        }
    }
}

pub struct PostgresStorage {
    pub pool: Pool,
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

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Object> {
        let client = self.pool.get().await?;

        Ok(client)
    }

    pub async fn get_db_tables(&self) -> Result<Vec<Table>>{
        let mut table_vec: Vec<Table> = Vec::new();
        let this_client = self.get_client().await.expect("Unable to retrieve Postgres Client");
        // Search for tables
        for tables_row in this_client.query(
            "SELECT table_catalog as db_name, table_schema,table_name
            FROM information_schema.tables
            WHERE table_schema in ('movies','public');",
            &[]).await.unwrap() {
            let table_schema : String = tables_row.get(1);
            let table_name : String = tables_row.get(2);

            let mut attributes_vec: Vec<Attribute> = Vec::new();


            // For each table, search for its attributes
            for attributes_row in this_client.query("
                SELECT column_name, data_type
                FROM information_schema.columns
                WHERE table_schema = 'movies' AND table_name = $1;
            ",&[&table_name]).await.unwrap(){

                let column_name : String = attributes_row.get(0);
                let data_type : String = attributes_row.get(1);
                let attribute : Attribute = Attribute::new(column_name,data_type);

                attributes_vec.push(attribute);
            }

            let table : Table = Table::new(table_schema,table_name,attributes_vec);
            table_vec.push(table);
        }

        Ok(table_vec)
    }

    pub async fn return_result(&self) -> &str{
        return "Rebolarion";
    }
}

//-----------------------------------------
// Should the following section of this file be refactored into it's own module?
//------------------------------------------


#[derive(Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    data_type: String
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    schema: String,
    name: String,
    attributes: Vec<Attribute>
}

#[derive(Serialize, Deserialize)]
struct ForeignKey {
    attribute1: Attribute,
    table1: Table,
    attribute2: Attribute,
    table2: Table
}

#[derive(Serialize, Deserialize)]
struct PrimaryKey {
    table: Table,
    attribute: Attribute,
}

#[derive(Serialize, Deserialize)]
struct DbSchema {
    tables: Vec<Table>,
    foreing_keys : Vec<ForeignKey>,
    primary_keys: Vec<PrimaryKey>
}

impl Attribute {
    pub fn new(arg_name:String,arg_type:String) -> Self {
        let name = arg_name;
        let data_type = arg_type;
        Self {name,data_type}
    }
}

impl Table {
    pub fn new(arg_schema:String,arg_name:String,arg_attributes:Vec<Attribute>) -> Self {
        let schema = arg_schema;
        let name = arg_name;
        let attributes = arg_attributes;
        Self {
            schema,
            name,
            attributes
        }
    }
}