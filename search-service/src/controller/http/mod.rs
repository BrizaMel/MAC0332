use axum::{extract::State, routing::get, response::IntoResponse, Router, Json};
use std::net::SocketAddr;

use std::sync::Arc;

use crate::traits::DatabaseOperations;

use crate::database_storage::DatabaseStorage;

pub struct AppState {
    db: DatabaseStorage,
}

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    // Choose correct Storage class, depending on the aplication DBMS 
    let storage = DatabaseStorage::new().await;

    let app_state = Arc::new(AppState { db: storage });

    let router = Router::new()
        .route("/properties", get(get_filter_properties))
        .route("/search", get(search()))
        .with_state(app_state);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_filter_properties(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {

    let db_storage = &app_state.db;

    let db_schema_info = db_storage.get_db_schema_info().await.expect("Error retireving Database Schema Information");
    
    let json_response = serde_json::json!({
        "status": "success",
        "schema_info": serde_json::json!(db_schema_info),
    });

    Json(json_response)
}

fn search() -> String {
    "search handler".to_string()
}
