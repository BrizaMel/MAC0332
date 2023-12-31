use anyhow::anyhow;
use axum::Extension;
use axum::{
    extract::Json, http::StatusCode, response::IntoResponse, routing::get, routing::post, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};

use crate::manager::SearchServiceManager;
use crate::storage::mysql::{MySQLConfig, MySQLStorage};
use crate::storage::postgres::{PostgresConfig, PostgresStorage};
use crate::traits::SearchServiceStorage;

use self::entities::{RequestError, Response, SearchRequest};

pub mod entities;

pub async fn run_http_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("provide a valid address");

    let storage = get_storage().await?;
    let manager = SearchServiceManager::new(storage).await;

    let get_filter_properties = Router::new().route("/properties", get(get_filter_properties));
    let search = Router::new().route("/search", post(search));

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .merge(get_filter_properties)
        .merge(search)
        .layer(Extension(manager))
        .layer(cors);

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
    Extension(manager): Extension<SearchServiceManager>,
) -> Result<impl IntoResponse, RequestError> {
    let properties = manager
        .get_filter_properties()
        .await
        .map_err(|e| RequestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        })?;

    let res = serde_json::json!({
        "properties": serde_json::json!(properties),
    });

    Ok(Response::new(StatusCode::OK, res))
}

async fn search(
    Extension(manager): Extension<SearchServiceManager>,
    Json(payload): Json<SearchRequest>,
) -> Result<Response<String>, RequestError> {
    let SearchRequest {
        projection,
        filters,
    } = payload;

    let res = manager.search(projection, filters).await?;
    let res = serde_json::to_string(&res).map_err(|_| RequestError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "could not serialize response".into(),
    })?;
    Ok(Response::new(StatusCode::OK, res))
}
