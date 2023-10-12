use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: String,
    username: String,
    language_code: String,
}

#[derive(Serialize, Deserialize)]
struct Chat {
    id: i64,
    first_name: String,
    last_name: String,
    username: String,
    r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Entity {
    offset: i64,
    length: i64,
    r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message_id: u32,
    message_thread_id: u32,
    from: User,
    chat: Chat,
    date: i64,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUpdate {
    update_id: u64,
    message: Message,
    entrities: Vec<Entity>,
}
