use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;

use std::sync::Arc;

use crate::postgres::{PostgresConfig,PostgresStorage};

pub struct AppState {
    db: PostgresStorage,
}

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    let storage = PostgresStorage::new(PostgresConfig::new()).await?;

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

async fn get_filter_properties(State(app_state): State<Arc<AppState>>) -> Result<String,String> {

    let db_storage = &app_state.db;

    Ok(db_storage.return_result().await.to_string())

    // Ok("abacaxi".to_string())
}

fn search() -> String {
    "search handler".to_string()
}
