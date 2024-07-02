use axum::Json;
use crate::application::content_service::ContentService;
use crate::infrastructure::content_repository::ContentRepository;
use axum::{response::IntoResponse,extract::{Path, Extension}};
use std::sync::Arc;

    
    pub async fn get_content_handler( Extension(content_repository): Extension<Arc<ContentRepository>>,)->impl IntoResponse{
        match ContentService::get_content(&content_repository).await {
            Ok(contents) => Json(contents).into_response(),
            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching contents").into_response(),
        }      
    }

    pub async fn get_content_by_id_handler(Path(id): Path<i32>, Extension(content_repository): Extension<Arc<ContentRepository>>)-> impl IntoResponse{
        match ContentService::get_content_by_id(&content_repository,id).await {
            Ok(contents) => Json(contents).into_response(),
            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching contents").into_response(),
        }
    }

