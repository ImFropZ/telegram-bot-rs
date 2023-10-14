mod controllers;
mod models;
mod tests;

mod telegram_bot {
    use crate::controllers::authentication::{Authentication, AuthenticationService};
    use crate::controllers::message::{Message, MessageService};
    use crate::models::get_update::GetUpdate;
    use crate::models::types::message::{Message as MessageType, SendMessage};
    use crate::models::types::user::User;
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
        async fn authenticate(&self) -> Result<User, Error>;
        async fn get_chats(&self) -> Result<Vec<GetUpdate>, Error>;
        async fn send_message(&self, send_message: SendMessage) -> Result<MessageType, Error>;
    }

    #[async_trait]
    impl TelegramBotService for TelegramBot {
        fn new(app_id: String) -> Self {
            TelegramBot { app_id }
        }

        fn get_app_id(&self) -> String {
            return self.app_id.clone();
        }

        async fn authenticate(&self) -> Result<User, Error> {
            Authentication::new(self).authenticate().await
        }

        async fn get_chats(&self) -> Result<Vec<GetUpdate>, Error> {
            let endpoint = format!("https://api.telegram.org/bot{}/getUpdates", self.app_id);
            let response = reqwest::Client::new().get(&endpoint).send().await?;

            return response.json::<Vec<GetUpdate>>().await;
        }

        async fn send_message(&self, send_message: SendMessage) -> Result<MessageType, Error> {
            Message::new(self).send_message(send_message).await
        }
    }
}
