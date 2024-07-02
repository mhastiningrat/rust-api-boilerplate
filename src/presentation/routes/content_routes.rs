use axum::{extract::{Path, Extension}, routing::get, Json, Router};
use crate::application::content_service::ContentService;
use crate::{
    infrastructure::content_repository::ContentRepository,
    infrastructure::author_repository::AuthorRepository};
use crate::presentation::handler;
use sqlx::PgPool;
use axum::response::IntoResponse;
use std::sync::Arc;

pub fn content_routes(pool: Arc<PgPool>) -> Router {
    let content_repository = Arc::new(ContentRepository::new(pool.clone()));
 
    Router::new()
        .route("/content", get(handler::content_handler::get_content_handler
        ))
        .route("/content/:id", get(handler::content_handler::get_content_by_id_handler
        ))
        .layer(Extension::from(axum::Extension(content_repository)))

}
