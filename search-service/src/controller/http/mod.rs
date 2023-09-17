use axum::{routing::get, Router, response::IntoResponse, Json};
use std::net::SocketAddr;

use crate::postgres::PostgresConfig;
use crate::postgres::PostgresStorage;
use serde::{Deserialize, Serialize};

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    axum::Server::bind(&addr)
        .serve(build_routes().await.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn build_routes() -> Router {
    Router::new()
        .route("/properties", get(get_filter_properties))
        .route("/search", get(search()))
}

#[derive(Serialize, Deserialize)]
struct Attribute {
    name: String,
    data_type: String
}

#[derive(Serialize, Deserialize)]
struct Table {
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

async fn get_filter_properties() -> impl IntoResponse {

    let psql_config = PostgresConfig::new();
    let psql_storage = PostgresStorage::new(psql_config).await.expect("Error initializing psql_storage");
    let psql_client = PostgresStorage::get_client(&psql_storage).await.expect("Error getting postgres client");

    let mut table_vec: Vec<Table> = Vec::new();
    // Search for tables
    for tables_row in psql_client.query(
        "SELECT table_catalog as db_name, table_schema,table_name
        FROM information_schema.tables
        WHERE table_schema in ('movies','public');",
        &[]).await.unwrap() {
        let table_schema : String = tables_row.get(1);
        let table_name : String = tables_row.get(2);

        let mut attributes_vec: Vec<Attribute> = Vec::new();


        // For each table, search for its attributes
        for attributes_row in psql_client.query("
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

    let json_response = serde_json::json!({
        "status": "success",
        "data":serde_json::json!({
            "tables":serde_json::json!(table_vec),
            "foreing_keys":"".to_string(),
            "primary_keys":"".to_string()           
        })
    });

    Json(json_response)
}

fn search() -> String {
    "search handler".to_string()
}
