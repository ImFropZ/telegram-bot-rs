use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i32,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i32,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicReopened {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicClosed {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
