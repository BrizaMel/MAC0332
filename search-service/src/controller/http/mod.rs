use axum::{routing::get, Router};
use std::net::SocketAddr;

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
        .route("/properties", get(get_filter_properties()))
        .route("/search", get(search()))
}

fn get_filter_properties() -> String {
    "get filter properties handler".to_string()
}

fn search() -> String {
    "search handler".to_string()
}
