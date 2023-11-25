use anyhow::{Ok, Result};
use mysql::{Pool,PooledConn,OptsBuilder};
use std::time::Duration;

pub mod queries;
pub mod tests;

pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub allowed_schemas: Vec<String>,
}

impl MySQLConfig {
    pub fn new(allowed_schemas_string: String, db_host : String, db_port: u16, mysql_user: String, mysql_pass: String, mysql_db: String) -> Self {
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
}

pub struct MySQLStorage {
    pub pool: Pool,
    pub allowed_schemas: Vec<String>,
}

impl MySQLStorage {
    pub async fn new(config: MySQLConfig) -> Result<Self> {
        let mysql_opts = OptsBuilder::new()
        	.user(Some(config.user.to_owned()))
        	.pass(Some(config.password.to_owned()))
        	.ip_or_hostname(Some(config.host.to_owned()))
        	.tcp_port(config.port.to_owned())
        	.db_name(Some(config.dbname.to_owned()))
        	.tcp_connect_timeout(Some(Duration::new(10, 0)))
        	.read_timeout(Some(Duration::new(3, 0)));

        let pool = Pool::new(mysql_opts).expect("ERROSSSS");

        let allowed_schemas = config.allowed_schemas;

        println!("Allowed Schemas: {:?}", allowed_schemas);

        Ok(Self {
            pool,
            allowed_schemas,
        })
    }

    fn get_client(&self) -> Result<PooledConn> {
        let client = self.pool.get_conn()?;

        Ok(client)
    }

}