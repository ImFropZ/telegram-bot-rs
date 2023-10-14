use serde::{Deserialize, Serialize};

use super::photo_size::PhotoSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>,
    pub thumbnail: Option<PhotoSize>,
}
