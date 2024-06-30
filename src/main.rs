use crate::config::AppConfig;
use axum::{response::IntoResponse, routing::get, Json, Router};
use clap::Parser;
use dotenv::dotenv;
use std::sync::Arc;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());
    let addr = format!("{}:{}", config.app_host, config.app_port);

    let app = Router::new().route("/health", get(healthcheck));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!(
        "🚀 [{}] listening on {}",
        config.app_env,
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

pub async fn healthcheck() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
