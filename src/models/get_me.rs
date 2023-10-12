use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMe {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: String,
    username: String,
    language_code: String,
}
