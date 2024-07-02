use crate::config::AppConfig;
use axum::{response::IntoResponse,Json, Router,extract::Extension};
use clap::Parser;
use dotenv::dotenv;
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

use std::env;

use crate::presentation::routes;

mod domain;
mod application;
mod infrastructure;
mod presentation;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB NYA HARUS ADA LHO YA COK");
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await.expect("Failed to create pool.");
    let pool = Arc::new(pool);

    let config = Arc::new(AppConfig::parse());
    let addr = format!("{}:{}", config.app_host, config.app_port);

    // let app = Router::new().route("/health", get(healthcheck))
    // .route("/getdata", get(get_contents))
    // .layer(Extension(pool));
    let author_router = routes::author_routes::author_routes(pool.clone());
    let content_router = routes::content_routes::content_routes(pool.clone());


    let app = Router::new()
        .nest("/api", author_router)
        .nest("/api", content_router);

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