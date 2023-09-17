use axum::{extract::State, routing::get, response::IntoResponse, Router, Json};
use std::net::SocketAddr;

use std::sync::Arc;

use crate::postgres::{PostgresConfig,PostgresStorage};

pub struct AppState {
    db: PostgresStorage,
}

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    //Choose correct Storage classe, depending on the aplication DBMS 
    let storage = PostgresStorage::new(PostgresConfig::new()).await.expect("Error initializing psql_storage");

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

    let table_vec = db_storage.get_db_tables().await.expect("Error retireving Database Tables");

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
