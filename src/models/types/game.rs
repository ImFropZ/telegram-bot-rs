use serde::{Deserialize, Serialize};

use super::{animation::Animation, message::MessageEntity, photo_size::PhotoSize, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendGame {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackGame {
    pub user_id: i64,
    pub score: u32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetGameScore {
    pub user_id: i64,
    pub score: u32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGameHighScores {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: u32,
    pub user: User,
    pub score: u32,
}
