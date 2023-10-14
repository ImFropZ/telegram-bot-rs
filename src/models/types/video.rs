use serde::{Deserialize, Serialize};

use super::{photo_size::PhotoSize, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatEnded {
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatStarted {}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}
