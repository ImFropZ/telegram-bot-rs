use super::{message::MessageEntity, chat::Chat, user::User};

pub struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: u32,
    is_closed: bool,
    is_anonymous: bool,
    r#type: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<u32>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<u32>,
    close_date: Option<u32>,
}

pub struct PollAnswer {
    poll_id: String,
    voter_chat: Chat,
    user: User,
    option_ids: Vec<u32>,
}

pub struct PollOption {
    text: String,
    voter_count: u32,
}
