use serde::{Deserialize, Serialize};

use super::photo_size::PhotoSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    file_id: String,
    file_unique_id: String,
    width: i32,
    height: i32,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
