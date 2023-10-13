use super::photo_size::PhotoSize;

pub struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: u32,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u32>,
    thumbnail: Option<PhotoSize>,
}
