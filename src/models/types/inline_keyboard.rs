use super::{
    game::CallbackGame, login_url::LoginUrl,
    switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat, web_app_info::WebAppInfo,
};

pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

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
