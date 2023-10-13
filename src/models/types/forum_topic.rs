pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i32,
    pub icon_custom_emoji_id: Option<String>,
}

pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i32,
    pub icon_custom_emoji_id: Option<String>,
}

pub struct ForumTopicReopened {}

pub struct ForumTopicClosed {}

pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
