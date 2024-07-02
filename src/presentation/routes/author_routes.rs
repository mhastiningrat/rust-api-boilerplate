use axum::{extract::Extension, routing::get, Router};
use crate::infrastructure::author_repository::AuthorRepository;
use crate::presentation::handler;
use sqlx::PgPool;
use std::sync::Arc;

pub fn author_routes(pool: Arc<PgPool>) -> Router {
    let author_repository = Arc::new(AuthorRepository::new(pool.clone()));

    Router::new()
        .route("/author", get(handler::author_handler::get_author_handler))
        .layer(Extension::from(axum::Extension(author_repository)))
}
