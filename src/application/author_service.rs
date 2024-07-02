use crate::domain::author::Author;
use crate::infrastructure::author_repository::AuthorRepository;

pub struct AuthorService;

impl AuthorService {

    pub async fn get_author(author_repository: &AuthorRepository) -> Result<Vec<Author>, sqlx::Error> {
       author_repository.get_author().await
    }
}
