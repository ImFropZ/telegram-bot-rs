use serde::{Deserialize, Serialize};

use super::{chat::Chat, message::MessageEntity, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: u32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub r#type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u32>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u32>,
    pub close_date: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Chat,
    pub user: User,
    pub option_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: u32,
}
