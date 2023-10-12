use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TelegramResponse<T> {
    ok: bool,
    result: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct TelegramErrorResponse {
    ok: bool,
    error_code: i32,
    description: String,
}
