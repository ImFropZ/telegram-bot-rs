use super::keyboard_button::KeyboardButton;

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
