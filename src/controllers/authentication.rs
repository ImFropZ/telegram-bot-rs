use crate::{
    models::{telegram_response::TelegramResponse, types::user::User},
    telegram_bot::{TelegramBot, TelegramBotService},
};
use async_trait::async_trait;
use reqwest::Error;

pub struct Authentication<'a> {
    telegram_bot: &'a TelegramBot,
}

#[async_trait]
pub trait AuthenticationService<'a> {
    fn new(telegram_bot: &'a TelegramBot) -> Authentication<'a>;
    async fn authenticate(&self) -> Result<User, Error>;
}

#[async_trait]
impl<'a> AuthenticationService<'a> for Authentication<'a> {
    fn new(telegram_bot: &'a TelegramBot) -> Self {
        Authentication { telegram_bot }
    }

    async fn authenticate(&self) -> Result<User, Error> {
        let endpoint = format!(
            "https://api.telegram.org/bot{}/getMe",
            self.telegram_bot.get_app_id()
        );

        let response = reqwest::Client::new().get(&endpoint).send().await.unwrap();
        let telegram_response_getme = response.json::<TelegramResponse<User>>().await?;

        return Ok(telegram_response_getme.result);
    }
}
