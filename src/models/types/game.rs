use super::{message_entity::MessageEntity, animation::Animation, photo_size::PhotoSize};

pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

pub struct SendGame {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

pub struct CallbackGame {
    pub user_id: i64,
    pub score: u32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

pub struct SetGameScore {
    pub user_id: i64,
    pub score: u32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

pub struct GetGameHighScores {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

pub struct GameHighScore {
    pub position: u32,
    pub user: User,
    pub score: u32,
}
