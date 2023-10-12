mod controllers;
mod models;
mod tests;

mod telegram_bot {
    use crate::controllers::authentication::{Authentication, AuthenticationService};
    use crate::models::{get_me::GetMe, get_update::GetUpdate};
    use async_trait::async_trait;
    use reqwest;
    use reqwest::Error;

    pub struct TelegramBot {
        app_id: String,
    }

    #[async_trait]
    pub trait TelegramBotService {
        fn new(app_id: String) -> TelegramBot;
        fn get_app_id(&self) -> String;
        async fn authenticate(&self) -> Result<GetMe, Error>;
        async fn get_chats(&self) -> Result<Vec<GetUpdate>, Error>;
    }

    #[async_trait]
    impl TelegramBotService for TelegramBot {
        fn new(app_id: String) -> TelegramBot {
            TelegramBot { app_id }
        }

        fn get_app_id(&self) -> String {
            return self.app_id.clone();
        }

        async fn authenticate(&self) -> Result<GetMe, Error> {
            Authentication::new(self).authenticate().await
        }

        async fn get_chats(&self) -> Result<Vec<GetUpdate>, Error> {
            let endpoint = format!("https://api.telegram.org/bot{}/getUpdates", self.app_id);
            let response = reqwest::Client::new().get(&endpoint).send().await?;

            return response.json::<Vec<GetUpdate>>().await;
        }
    }
}
