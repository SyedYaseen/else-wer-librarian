use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthorJson {
    pub key: String,          // "/authors/OL10000050A"
    pub name: Option<String>, // "Günter Brauneis"
}

#[derive(Debug, Deserialize)]
pub struct WorkJson {
    pub key: String,           // "/works/OL10000177W"
    pub title: Option<String>, // "L'île des esclaves, de Marivaux"
    pub authors: Option<Vec<WorkAuthorRef>>,
}

#[derive(Debug, Deserialize)]
pub struct WorkAuthorRef {
    pub author: Option<AuthorRef>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorRef {
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct EditionRecord {
    pub key: String,
    pub title: String,

    #[serde(default)]
    pub authors: Vec<AuthorRef>,

    #[serde(default)]
    pub series: Vec<String>,

    #[serde(default)]
    pub covers: Vec<i64>,
}

impl EditionRecord {
    pub fn normalize(&mut self) {
        if let Some(stripped) = self.key.strip_prefix("/books/") {
            self.key = stripped.to_string();
        }
        for author in &mut self.authors {
            if let Some(stripped) = author.key.strip_prefix("/authors/") {
                author.key = stripped.to_string();
            }
        }
    }
}
