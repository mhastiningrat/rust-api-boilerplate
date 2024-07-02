use crate::domain::author::Author;
use sqlx::{PgPool, query_as};
use std::sync::Arc;

pub struct AuthorRepository {
    pool: Arc<PgPool>,
}

impl AuthorRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_author(&self) -> Result<Vec<Author>, sqlx::Error> {
        let row = sqlx::query_as!(
            Author,
            r#"
            SELECT  firstname,
                    lastname
            FROM authors
            "#
        )
        .fetch_all(&*self.pool)
        .await?;
        
        Ok(row)
    }

   
}
