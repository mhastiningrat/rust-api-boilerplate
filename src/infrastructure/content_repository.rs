use crate::domain::content::Content;
use sqlx::{PgPool, query_as};
use std::sync::Arc;

pub struct ContentRepository {
    pool: Arc<PgPool>,
}

impl ContentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_content(&self) -> Result<Vec<Content>, sqlx::Error> {
        let row = sqlx::query_as!(
            Content,
            r#"
            SELECT  id,
                    title,
                    slug,
                    thumbnail,
                    image,
                    description,
                    body
            FROM content
            "#
        )
        .fetch_all(&*self.pool)
        .await?;
        
        Ok(row)
    }

    pub async fn get_content_by_id(&self,id:i32) -> Result<Content, sqlx::Error> {
        let row = sqlx::query_as!(
            Content,
            r#"
            SELECT  id,
                    title,
                    slug,
                    thumbnail,
                    image,
                    description,
                    body
            FROM content 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        
        Ok(row)
    }
}
