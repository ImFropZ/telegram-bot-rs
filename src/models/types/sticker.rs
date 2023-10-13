use super::{photo_size::PhotoSize, file::File};

pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub r#type: String,
    pub width: i32,
    pub height: i32,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<Box<PhotoSize>>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<i32>,
}

pub struct MaskPosition {
    pub point: String,
    pub x_shift: f32,
    pub y_shift: f32,
    pub scale: f32,
}

