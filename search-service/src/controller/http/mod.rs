use anyhow::anyhow;
use axum::Extension;
use axum::{
    extract::Json, http::StatusCode, response::IntoResponse, routing::get, routing::post, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::query_representation::initial::initial_to_command;
use crate::relational::entities::DbSchema;
use crate::relational::table_search::entities::TableSearchInfo;
use crate::relational::table_search::TableSearch;
use crate::storage::mysql::{MySQLConfig, MySQLStorage};
use crate::storage::postgres::{PostgresConfig, PostgresStorage};
use crate::storage::DatabaseVisitor;
use crate::traits::{Component, SearchServiceStorage};

use self::errors::RequestError;

pub mod errors;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    projection: Vec<String>,
    filters: String,
}

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    // Choose correct Storage class, depending on the aplication DBMS
    let storage = get_storage().await?;

    let get_filter_properties = Router::new().route("/properties", get(get_filter_properties));
    let search = Router::new().route("/search", post(search));

    let router = Router::new()
        .merge(get_filter_properties)
        .merge(search)
        .layer(Extension(storage));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_storage() -> anyhow::Result<Arc<dyn SearchServiceStorage>> {
    let dbsm_to_connect = std::env::var("DBMS").expect("DBMS variable missing in the environment");

    let storage: Arc<dyn SearchServiceStorage> = match dbsm_to_connect.as_str() {
        "postgres" => Arc::new(PostgresStorage::new(PostgresConfig::from_env()).await?),
        "mysql" => Arc::new(MySQLStorage::new(MySQLConfig::from_env()).await?),
        _ => return Err(anyhow!("this database is no available for use")),
    };

    Ok(storage)
}

async fn get_filter_properties(
    Extension(storage): Extension<Arc<dyn SearchServiceStorage>>,
) -> impl IntoResponse {
    let db_schema_info = &storage
        .get_db_schema_info()
        .await
        .expect("Error retireving Database Schema Information");

    let json_response = serde_json::json!({
        "status": "success",
        "schema_info": serde_json::json!(db_schema_info),
    });

    Json(json_response)
}

async fn search(
    Extension(storage): Extension<Arc<dyn SearchServiceStorage>>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<String>, RequestError> {
    let SearchRequest {
        projection,
        filters,
    } = payload;

    let command = initial_to_command(filters).map_err(|_| RequestError {
        status_code: StatusCode::BAD_REQUEST,
        message: "failed to parse request".into(),
    })?;

    // get database schema info, such as tables and foreign keys
    let db_schema_info = &storage
        .get_db_schema_info()
        .await
        .map_err(|_| RequestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "could not retrieve database schema info".into(),
        })?;

    let DbSchema {
        tables,
        foreign_keys,
    } = db_schema_info;
    let tables_search_info: Vec<TableSearchInfo> = tables
        .clone()
        .into_iter()
        .map(TableSearchInfo::from)
        .collect();

    // create graph to represent links between tables
    let table_search: TableSearch = TableSearch::new(tables_search_info, foreign_keys.clone());

    // create a visitor that turns a command into a query
    let visitor = DatabaseVisitor::new(table_search);

    let query = command
        .accept(projection, Arc::new(visitor))
        .map_err(|_| RequestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "failed to build query".into(),
        })?;

    let res = storage.execute(query).await.map_err(|_| RequestError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "failed to execute query".into(),
    })?;
    let res = serde_json::to_string(&res).map_err(|_| RequestError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "failed to serialize response".into(),
    })?;

    Ok(Json(res))
}
