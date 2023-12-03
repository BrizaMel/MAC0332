use axum::{
    extract::Json, extract::State, http::StatusCode, response::IntoResponse, routing::get,
    routing::post, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::database_storage::visitor::DatabaseVisitor;
use crate::database_storage::DatabaseStorage;
use crate::query_representation::initial::initial_to_command;
use crate::relational::entities::DbSchema;
use crate::relational::table_search::entities::TableSearchInfo;
use crate::relational::table_search::TableSearch;
use crate::traits::{Component, DatabaseOperations};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    projection: Vec<String>,
    filters: String,
}

pub struct AppState {
    db: DatabaseStorage,
}

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    // Choose correct Storage class, depending on the aplication DBMS
    let storage = DatabaseStorage::new().await;

    let app_state = Arc::new(AppState { db: storage });

    let get_filter_properties = Router::new().route("/properties", get(get_filter_properties));
    let search = Router::new().route("/search", post(search));

    let router = Router::new()
        .merge(get_filter_properties)
        .merge(search)
        .with_state(app_state);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_filter_properties(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_storage = &app_state.db;

    let db_schema_info = db_storage
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
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<String>, RequestError> {
    println!("search new version");
    let SearchRequest {
        projection,
        filters,
    } = payload;

    let command = initial_to_command(filters).map_err(|_| RequestError {
        status_code: StatusCode::BAD_REQUEST,
        message: "failed to parse request".into(),
    })?;

    // get database schema info, such as tables and foreign keys
    let db_storage = &app_state.db;
    let db_schema_info = db_storage
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
    let tables_search_info: Vec<TableSearchInfo> =
        tables.into_iter().map(TableSearchInfo::from).collect();

    // create graph to represent links between tables
    let table_search: TableSearch = TableSearch::new(tables_search_info, foreign_keys);

    // create a visitor that turns a command into a query
    let visitor = DatabaseVisitor::new(table_search);

    let query = command
        .accept(projection, Arc::new(visitor))
        .map_err(|_| RequestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "failed to build query".into(),
        })?;

    Ok(Json(query))
}
