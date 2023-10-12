mod models;
mod tests;

mod telegram_bot {
    use crate::models::{
        get_me::GetMe, get_update::GetUpdate, telegram_response::TelegramResponse,
    };
    use async_trait::async_trait;
    use reqwest;
    use reqwest::Error;

    pub struct TelegramBot {
        app_id: String,
    }

    #[async_trait]
    pub trait TelegramBotService {
        fn new(app_id: String) -> TelegramBot;
        async fn authenticate(&self) -> Result<TelegramResponse<GetMe>, Error>;
        async fn get_chats(&self) -> Result<TelegramResponse<Vec<GetUpdate>>, Error>;
    }

    #[async_trait]
    impl TelegramBotService for TelegramBot {
        fn new(app_id: String) -> TelegramBot {
            TelegramBot { app_id }
        }

        async fn authenticate(&self) -> Result<TelegramResponse<GetMe>, Error> {
            let endpoint = format!("https://api.telegram.org/bot{}/getMe", self.app_id);
            let response = reqwest::Client::new().get(&endpoint).send().await.unwrap();

            let getme = response.json::<TelegramResponse<GetMe>>().await?;
            Ok(getme)
        }

        async fn get_chats(&self) -> Result<TelegramResponse<Vec<GetUpdate>>, Error> {
            let endpoint = format!("https://api.telegram.org/bot{}/getUpdates", self.app_id);
            let response = reqwest::Client::new().get(&endpoint).send().await?;

            return response.json::<TelegramResponse<Vec<GetUpdate>>>().await;
        }
    }
}
