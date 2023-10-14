use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramErrorResponse {
    pub ok: bool,
    pub error_code: i32,
    pub description: String,
}
