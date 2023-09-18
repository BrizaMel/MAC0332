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

    pub async fn get_db_schema_info(&self) -> Result<DbSchema>{

        let table_vec = self.get_db_tables().await.expect("Error retireving Database Tables");
        let foreign_key_vec = self.get_db_foreign_keys().await.expect("Error retireving Database Foreign Keys");
        let db_schema : DbSchema = DbSchema::new(table_vec,foreign_key_vec);

        Ok(db_schema)
    }

    async fn get_db_tables(&self) -> Result<Vec<Table>>{
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

            let mut primary_keys_vec: Vec<PrimaryKey> = Vec::new();

            // For each table, search for its primary_keys
            for primary_keys_row in this_client.query("
                SELECT tc.table_schema,tc.table_name,c.column_name
                FROM information_schema.table_constraints tc 
                JOIN information_schema.constraint_column_usage AS ccu USING (constraint_schema, constraint_name) 
                JOIN information_schema.columns AS c ON c.table_schema = tc.constraint_schema
                  AND tc.table_name = c.table_name AND ccu.column_name = c.column_name
                WHERE constraint_type = 'PRIMARY KEY' AND tc.table_schema in ('public','movies')
                AND tc.table_name = $1;
            ",&[&table_name]).await.unwrap(){

                let schema_name = primary_keys_row.get(0);
                let table_name = primary_keys_row.get(1);
                let attribute_name = primary_keys_row.get(2);
                let primary_key : PrimaryKey = PrimaryKey::new(
                    schema_name,
                    table_name,
                    attribute_name);
                primary_keys_vec.push(primary_key);
            }

            let table : Table = Table::new(table_schema,table_name,attributes_vec,primary_keys_vec);
            table_vec.push(table);
        }

        Ok(table_vec)
    }

    async fn get_db_foreign_keys(&self) -> Result<Vec<ForeignKey>>{
        let mut foreign_keys_vec: Vec<ForeignKey> = Vec::new();
        let this_client = self.get_client().await.expect("Unable to retrieve Postgres Client");

        // Search for foreign keys
        for foreign_keys_rows in this_client.query("
            SELECT
                tc.table_schema, 
                tc.table_name, 
                kcu.column_name, 
                ccu.table_schema AS foreign_table_schema,
                ccu.table_name AS foreign_table_name,
                ccu.column_name AS foreign_column_name 
            FROM information_schema.table_constraints AS tc 
            JOIN information_schema.key_column_usage AS kcu
                ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage AS ccu
                ON ccu.constraint_name = tc.constraint_name
            WHERE tc.constraint_type = 'FOREIGN KEY';
        ",&[]).await.unwrap(){
            let schema_name: String = foreign_keys_rows.get(0);
            let table_name: String =foreign_keys_rows.get(1);
            let attribute_name: String = foreign_keys_rows.get(2);
            let schema_name_foreign: String = foreign_keys_rows.get(3);
            let table_name_foreign: String = foreign_keys_rows.get(4);
            let attribute_name_foreign: String = foreign_keys_rows.get(5);
    
            let foreign_key : ForeignKey = ForeignKey::new(
                schema_name,
                table_name,
                attribute_name,
                schema_name_foreign,
                table_name_foreign,
                attribute_name_foreign);
            foreign_keys_vec.push(foreign_key);

        }

        Ok(foreign_keys_vec)
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
    attributes: Vec<Attribute>,
    primary_keys: Vec<PrimaryKey>
}

#[derive(Serialize, Deserialize)]
pub struct ForeignKey {
    schema_name: String,
    table_name: String,
    attribute_name: String,
    schema_name_foreign: String,
    table_name_foreign: String,
    attribute_name_foreign: String
}

#[derive(Serialize, Deserialize)]
pub struct PrimaryKey {
    schema_name: String,
    table_name: String,
    attribute_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbSchema {
    tables: Vec<Table>,
    foreing_keys : Vec<ForeignKey>,
}

impl Attribute {
    pub fn new(arg_name:String,arg_type:String) -> Self {
        let name = arg_name;
        let data_type = arg_type;
        Self {name,data_type}
    }
}

impl Table {
    pub fn new(schema:String,name:String,attributes:Vec<Attribute>,primary_keys:Vec<PrimaryKey>) -> Self {
        Self {
            schema,
            name,
            attributes,
            primary_keys
        }
    }
}

impl ForeignKey {
    pub fn new(
            schema_name:String,
            table_name:String,
            attribute_name:String,
            schema_name_foreign:String,
            table_name_foreign:String,
            attribute_name_foreign:String) -> Self {
        Self {
            schema_name,
            table_name,
            attribute_name,
            schema_name_foreign,
            table_name_foreign,
            attribute_name_foreign
        }
    }
}

impl PrimaryKey {
    pub fn new(
            schema_name:String,
            table_name:String,
            attribute_name:String) -> Self {
        Self {
            schema_name,
            table_name,
            attribute_name,
        }
    }
}

impl DbSchema {
    pub fn new(
            tables:Vec<Table>,
            foreing_keys:Vec<ForeignKey>) -> Self {
        Self {
            tables,
            foreing_keys
        }
    }
}