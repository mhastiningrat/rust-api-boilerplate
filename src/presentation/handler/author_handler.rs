use axum::Json;
use crate::{
    application::author_service::AuthorService,
    infrastructure::author_repository::AuthorRepository
};
use sqlx::PgPool;
use axum::{response::IntoResponse,extract::{Path, Extension}};
use std::sync::Arc;

    
    pub async fn get_author_handler( Extension(author_repository): Extension<Arc<AuthorRepository>>,)->impl IntoResponse{
               
   
                        match AuthorService::get_author(&author_repository).await {
                            Ok(contents) => Json(contents).into_response(),
                            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching contents").into_response(),
                        }
                    
                
    }
