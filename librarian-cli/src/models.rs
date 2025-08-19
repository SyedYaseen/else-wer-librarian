use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseFileMetadata {
    pub book_id: i64,
    pub file_id: Option<i64>,
    pub file_name: String,
    pub file_path: String,
    pub duration: Option<i64>,
    pub channels: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: i64,
    #[serde(flatten)]
    pub data: BaseFileMetadata,
}

pub type CreateFileMetadata = BaseFileMetadata;

impl CreateFileMetadata {
    pub fn new(
        file_path: String,
        file_id: Option<i64>,
        file_name: String,
        duration: Option<i64>,
        channels: Option<i64>,
        sample_rate: Option<i64>,
        bitrate: Option<i64>,
    ) -> CreateFileMetadata {
        CreateFileMetadata {
            book_id: -99,
            file_id: file_id,
            file_name: file_name,
            file_path: file_path,
            duration: duration,
            channels: channels,
            sample_rate: sample_rate,
            bitrate: bitrate,
        }
    }
}
