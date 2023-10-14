use serde::{Deserialize, Serialize};

use super::{
    chat::ChatAdministratorRights, force_reply::ForceReply, game::CallbackGame,
    login_url::LoginUrl, switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat,
    web_app::WebAppInfo,
};

pub enum ReplayMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplayKeyboardMarkup(ReplayKeyboardMarkup),
    ReplayKeyboardRemove(ReplayKeyboardRemove),
    ForceReply(ForceReply),
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

pub struct ReplayKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

pub struct ReplayKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}
