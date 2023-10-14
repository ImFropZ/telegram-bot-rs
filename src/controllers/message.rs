use async_trait::async_trait;
use reqwest::{ClientBuilder, Error};

use crate::{
    models::{
        telegram_response::{self, TelegramResponse},
        types::message::{Message as MessageType, SendMessage},
    },
    telegram_bot::{TelegramBot, TelegramBotService},
};

pub struct Message<'a> {
    telegram_bot: &'a TelegramBot,
}

#[async_trait]
pub trait MessageService<'a> {
    fn new(telegram_bot: &'a TelegramBot) -> Self;
    async fn send_message(&self, send_message: SendMessage) -> Result<MessageType, Error>;
}

#[async_trait]
impl<'a> MessageService<'a> for Message<'a> {
    fn new(telegram_bot: &'a TelegramBot) -> Self {
        Message { telegram_bot }
    }

    async fn send_message(&self, send_message: SendMessage) -> Result<MessageType, Error> {
        let client = reqwest::Client::builder().build()?;

        let endpoint = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.telegram_bot.get_app_id(),
        );

        let request = client
            .request(reqwest::Method::POST, endpoint)
            .json(&serde_json::json!({
                "chat_id": send_message.chat_id.to_string(),
                "text": "Hello World",
            }));

        let response = request.send().await?;
        let telegram_response_message: TelegramResponse<MessageType> = response.json().await?;

        print!("{:?}", telegram_response_message);

        return Ok(telegram_response_message.result);
    }
}
