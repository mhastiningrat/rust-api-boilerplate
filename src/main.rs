use crate::config::AppConfig;
use axum::{response::IntoResponse, routing::get, Json, Router,http::StatusCode,extract::Extension,};
use serde::{Deserialize, Serialize};
use clap::Parser;
use dotenv::dotenv;
use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::env;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB NYA HARUS ADA LHO YA COK");
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await.expect("Failed to create pool.");

    let config = Arc::new(AppConfig::parse());
    let addr = format!("{}:{}", config.app_host, config.app_port);

    let app = Router::new().route("/health", get(healthcheck))
    .route("/getdata", get(get_contents))
    .layer(Extension(pool));

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

#[derive(Serialize)]
struct Contents {
    id:i32,
    title:String,
    slug:String,
    thumbnail:String,
    image:String,
    description:String,
    body:String,
}

async fn get_contents(
    Extension(pool): Extension<sqlx::PgPool>,
) -> impl IntoResponse {
    let users = sqlx::query_as!(
        Contents,
        r#"
        SELECT id,
        title,
        slug,
        thumbnail,
        image,
        description,
        body
        FROM content
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(users)
}
