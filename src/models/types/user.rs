use super::photo_size::PhotoSize;

pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}

pub struct UserShared {
    pub request_id: i32,
    pub user_id: i32,
}

pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}
