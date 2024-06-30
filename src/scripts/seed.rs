use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{Error, PgPool};
use std::env;
use std::fs;

#[derive(Deserialize)]
struct Author {
    firstname: String,
    lastname: String,
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    slug: String,
}

#[derive(Deserialize)]
struct Content {
    title: String,
    slug: String,
    thumbnail: String,
    image: String,
    description: String,
    body: String,
    author: i32,
    tags: Vec<i32>,
    published: bool,
}

#[derive(Deserialize)]
struct SeedData {
    authors: Vec<Author>,
    tags: Vec<Tag>,
    content: Vec<Content>,
}

async fn seed_data(pool: &PgPool) -> Result<(), Error> {
    let data = fs::read_to_string("data.json").expect("Unable to read file");
    let seed_data: SeedData = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for author in seed_data.authors {
        sqlx::query!(
            r#"
            INSERT INTO authors (firstname, lastname)
            VALUES ($1, $2)
            "#,
            author.firstname,
            author.lastname
        )
        .execute(pool)
        .await?;
    }

    for tag in seed_data.tags {
        sqlx::query!(
            r#"
            INSERT INTO tags (name, slug)
            VALUES ($1, $2)
            "#,
            tag.name,
            tag.slug
        )
        .execute(pool)
        .await?;
    }

    for content in seed_data.content {
        let content_id = sqlx::query!(
            r#"
            INSERT INTO content (title, slug, thumbnail, image, description, body, author_id, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            content.title,
            content.slug,
            content.thumbnail,
            content.image,
            content.description,
            content.body,
            content.author,
            content.published
        )
        .fetch_one(pool)
        .await?
        .id;

        for tag_id in content.tags {
            sqlx::query!(
                r#"
                INSERT INTO content_tags (content_id, tag_id)
                VALUES ($1, $2)
                "#,
                content_id,
                tag_id
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    seed_data(&pool).await?;

    Ok(())
}
