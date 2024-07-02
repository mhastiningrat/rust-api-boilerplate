use crate::domain::content::Content;
use crate::infrastructure::content_repository::ContentRepository;

pub struct ContentService;

impl ContentService {

    pub async fn get_content(content_repository: &ContentRepository) -> Result<Vec<Content>, sqlx::Error> {
        content_repository.get_content().await
    }

    pub async fn get_content_by_id(content_repository: &ContentRepository,id:i32) -> Result<Content, sqlx::Error> {
        content_repository.get_content_by_id(id).await
    }
}
