mod db;
mod models;
use crate::db::{insert_author, insert_edition, insert_work};
use crate::models::{AuthorJson, EditionRecord, WorkJson};
use dotenv::dotenv;
use futures::stream::{self, StreamExt};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use walkdir::WalkDir;
struct FileJob {
    path: String,
    kind: FileKind,
}

enum FileKind {
    Authors,
    Works,
    Editions,
}

impl FileJob {
    async fn parse_line(&self, pool: &PgPool, line: &str) -> anyhow::Result<()> {
        match self.kind {
            FileKind::Authors => Self::parse_author(pool, line).await,
            FileKind::Works => Self::parse_work(pool, line).await,
            FileKind::Editions => Self::parse_edition(pool, line).await,
        }
    }

    async fn parse_author(pool: &PgPool, line: &str) -> anyhow::Result<()> {
        let json_part = line.splitn(5, '\t').nth(4).unwrap();
        let author: AuthorJson = serde_json::from_str(json_part)?;

        let id = author
            .key
            .trim_start_matches("/authors/")
            .trim()
            .to_string();
        let name = author.name.unwrap_or_default().trim().to_string();

        if let Err(e) = insert_author(&pool, id, name).await {
            eprintln!("Insert failed: {}", e);
        }
        Ok(())
    }

    async fn parse_work(pool: &PgPool, line: &str) -> anyhow::Result<()> {
        let json_part = line.splitn(5, '\t').nth(4).unwrap();
        let work: WorkJson = serde_json::from_str(json_part)?;

        let id = work.key.trim_start_matches("/works/").to_string();
        let title = work.title.unwrap_or_default();

        // Grab the first author if present
        let author_id = work
            .authors
            .as_ref()
            .and_then(|v| v.first())
            .and_then(|a| a.author.as_ref())
            .map(|a| a.key.trim_start_matches("/authors/").to_string());

        if let Err(e) = insert_work(
            &pool,
            id.clone(),
            author_id.clone().unwrap_or_default(),
            title.clone(),
        )
        .await
        {
            eprintln!(
                "Insert works failed: {}. Author: {}, title: {}, workid: {}",
                e,
                author_id.unwrap_or_default(),
                title,
                id
            );
        }
        Ok(())
    }

    async fn parse_edition(pool: &PgPool, line: &str) -> anyhow::Result<()> {
        let parts: Vec<&str> = line.splitn(5, '\t').collect();
        if parts.len() < 5 {
            eprintln!("Cant insert eidtion {}", parts[1]);
        }

        let json_str = parts[4];

        match serde_json::from_str::<EditionRecord>(json_str) {
            Ok(mut edition) => {
                edition.normalize();
                let edition_id = edition.key.trim_start_matches("/books/");
                let series = edition.series.get(0).map(|s| s.as_str());
                let cover = edition.covers.get(0).cloned().unwrap_or_default();
                let authors: Vec<String> = edition
                    .authors
                    .iter()
                    .map(|a| a.key.trim_start_matches("/authors/").to_string())
                    .collect();
                if let Some(series) = series {
                    if let Err(e) = insert_edition(
                        &pool,
                        edition_id.to_string(),
                        edition.title.to_string(),
                        series.to_string(),
                        cover,
                        &authors,
                    )
                    .await
                    {
                        // eprintln!(
                        //     "Err while inserting edition: {} Edition: {} Author: {}",
                        //     e,
                        //     edition_id,
                        //     &authors.join(", ")
                        // );
                    }
                }
            }
            Err(e) => {
                // eprintln!("Failed to parse edition JSON: {} {}", e, line);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&database_url).await?;

    // process_author(&pool).await?;
    let author = FileJob {
        path: "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/authors_dump"
            .to_string(),
        kind: FileKind::Authors,
    };

    let works = FileJob {
        path: "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/works_dump"
            .to_string(),
        kind: FileKind::Works,
    };

    let editions = FileJob {
        path: "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/editions_dump"
            .to_string(),
        kind: FileKind::Editions,
    };

    let author = Arc::new(author);
    let works = Arc::new(works);
    let editions = Arc::new(editions);

    // process_file(&pool, Arc::clone(&author)).await;
    // process_file(&pool, Arc::clone(&works)).await;
    process_file(&pool, Arc::clone(&editions)).await;

    Ok(())
}

async fn process_file(pool: &PgPool, job: Arc<FileJob>) -> anyhow::Result<()> {
    let path = job.path.clone();
    let entries = WalkDir::new(path).max_depth(1);

    stream::iter(entries)
        .filter_map(|entry| async move {
            match entry {
                Ok(ent) => Some(ent),
                Err(e) => {
                    eprintln!("Failed to read entry: {e}");
                    None
                }
            }
        })
        .map(|entry| {
            let pool = pool.clone();
            let f_path = entry.path().to_owned();
            let job = Arc::clone(&job);

            async move {
                let file = File::open(f_path).await?;
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await? {
                    job.parse_line(&pool, &line).await?;
                }

                Ok::<(), anyhow::Error>(())
            }
        })
        .buffered(200)
        .for_each(|res| async {
            if let Err(e) = res {
                // eprintln!("Error: {e}");
            }
        })
        .await;
    Ok(())
}

// async fn process_author(pool: &PgPool) -> anyhow::Result<()> {
//     let author_path =
//         "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/authors_dump";

//     let entries = WalkDir::new(author_path).max_depth(1);

//     stream::iter(entries)
//         .filter_map(|entry| async move {
//             match entry {
//                 Ok(ent) => Some(ent),
//                 Err(e) => {
//                     eprintln!("Failed to read entry: {e}");
//                     None
//                 }
//             }
//         })
//         .map(|entry| {
//             let pool = pool.clone();
//             let f_path = entry.path().to_owned();

//             async move {
//                 let file = File::open(f_path).await?;
//                 let reader = BufReader::new(file);
//                 let mut lines = reader.lines();

//                 while let Some(line) = lines.next_line().await? {
//                     let parsed = parse_author(&line)?;
//                     if let Err(e) = insert_author(&pool, parsed.0, parsed.1).await {
//                         eprintln!("Insert failed: {}", e);
//                     }
//                 }

//                 Ok::<(), anyhow::Error>(())
//             }
//         })
//         .buffered(16)
//         .for_each(|res| async {
//             if let Err(e) = res {
//                 eprintln!("Error: {e}");
//             }
//         })
//         .await;
//     Ok(())
// }

// async fn process_works(pool: &PgPool) -> anyhow::Result<()> {
//     let works = File::open(
//         "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/ol_dump_works.txt",
//     )?;
//     let reader = BufReader::new(works);

//     let mut log_file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("failed_works.log")?;

//     for line in reader.lines() {
//         let line = line?;
//         // println!("{}", line);
//         match parse_work(&line) {
//             Some(val) => {
//                 if let Err(e) = insert_work(&pool, val.0, val.1.unwrap_or_default(), val.2).await {
//                     eprintln!("Insert failed: {}", e);
//                     writeln!(log_file, "INSERT FAILED: {} | Error: {}", line, e)?;
//                 }
//             }
//             None => {
//                 eprintln!("Parse failed");
//                 writeln!(log_file, "PARSE FAILED: {}", line)?;
//             }
//         }
//     }

//     Ok(())
// }

// async fn process_edition(pool: &PgPool) -> anyhow::Result<()> {
//     let edition = File::open(
//         "/home/yaseen/Projects/else-wer-librarian/librarian-openlibrary/data/ol_dump_editions_2025-07-31.txt",
//     )?;
//     let reader = BufReader::new(edition);
//     let mut log_file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("failed_edition.log")?;

//     for line in reader.lines() {
//         let line = line?;
//         // println!("{}", line);
//         match parse_edition(&line) {
//             Some(edition) => {
//                 let edition_id = edition.key.trim_start_matches("/books/");
//                 let series = edition.series.get(0).map(|s| s.as_str());
//                 let cover = edition.covers.get(0).cloned();
//                 let authors: Vec<String> = edition
//                     .authors
//                     .iter()
//                     .map(|a| a.key.trim_start_matches("/authors/").to_string())
//                     .collect();

//                 if let Err(e) =
//                     insert_edition(&pool, edition_id, &edition.title, series, cover, &authors).await
//                 {
//                     writeln!(log_file, "INSERT FAILED: {} | Error: {}", line, e)?;
//                     eprintln!("Failed to insert edition {}: {}", edition_id, e);
//                 }
//             }
//             None => {
//                 writeln!(log_file, "PARSE FAILED: {}", line)?;
//             }
//         }
//     }
//     Ok(())
// }

// pub fn parse_edition(line: &str) -> Option<EditionRecord> {
//     let parts: Vec<&str> = line.splitn(5, '\t').collect();
//     if parts.len() < 5 {
//         return None;
//     }

//     let json_str = parts[4];

//     match serde_json::from_str::<EditionRecord>(json_str) {
//         Ok(mut record) => {
//             record.normalize();

//             Some(record)
//         }
//         Err(e) => {
//             eprintln!("Failed to parse edition JSON: {}", e);
//             None
//         }
//     }
// }

// fn parse_work(line: &str) -> anyhow::Result<(String, Option<String>, String)> {
//     let json_part = line.splitn(5, '\t').nth(4).unwrap();
//     let work: WorkJson = serde_json::from_str(json_part)?;

//     let id = work.key.trim_start_matches("/works/").to_string();
//     let title = work.title.unwrap_or_default();

//     // Grab the first author if present
//     let author_id = work
//         .authors
//         .as_ref()
//         .and_then(|v| v.first())
//         .and_then(|a| a.author.as_ref())
//         .map(|a| a.key.trim_start_matches("/authors/").to_string());

//     Ok((id, author_id, title))
// }

// fn parse_author(line: &str) -> anyhow::Result<(String, String)> {
//     // Last field is JSON
//     let json_part = line.splitn(5, '\t').nth(4).unwrap();
//     let author: AuthorJson = serde_json::from_str(json_part)?;

//     let id = author.key.trim_start_matches("/authors/").to_string();
//     let name = author.name.unwrap_or_default();
//     Ok((id, name))
// }
