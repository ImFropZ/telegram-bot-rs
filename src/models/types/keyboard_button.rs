use super::{chat::ChatAdministratorRights, web_app::WebAppInfo};

pub struct KeyboardButton {
    pub text: String,
    pub request_user: Option<KeyboardButtonRequestUser>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}

pub struct KeyboardButtonPollType {
    pub r#type: Option<String>,
}

pub struct KeyboardButtonRequestUser {
    pub request_id: i32,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
}

pub struct KeyboardButtonRequestChat {
    pub request_id: i32,
    pub chat_is_channel: bool,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
}
