use axum::{routing::get, Router};
use std::net::SocketAddr;

use std::sync::Arc;

use crate::database_storage::DatabaseStorage;

use crate::api::properties::get_filter_properties;

pub struct AppState {
    pub db: DatabaseStorage,
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

fn search() -> String {
    "search handler".to_string()
}