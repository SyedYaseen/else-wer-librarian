use sqlx::PgPool;

use crate::models::{AuthorJson, EditionRecord, WorkJson};

pub async fn insert_author(pool: &PgPool, id: String, name: String) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO authors (id, name) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        id,
        name
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------- Works ----------------------
pub async fn insert_work(
    pool: &PgPool,
    id: String,
    author_id: String,
    title: String,
) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO works (id, title, author_id) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
        id,
        title,
        author_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------- Editions ----------------------
pub async fn insert_edition(
    pool: &PgPool,
    edition_id: String,
    title: String,
    series: String,
    cover: i64,
    authors: &[String],
) -> anyhow::Result<()> {
    // Insert edition
    sqlx::query!(
        r#"
        INSERT INTO editions (id, title, series, cover)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO NOTHING
        "#,
        edition_id,
        title,
        series,
        cover
    )
    .execute(pool)
    .await?;

    // Insert many-to-many authors
    for author_id in authors {
        sqlx::query!(
            r#"
            INSERT INTO edition_authors (edition_id, author_id)
            VALUES ($1, $2)
            ON CONFLICT (edition_id, author_id) DO NOTHING
            "#,
            edition_id,
            author_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
