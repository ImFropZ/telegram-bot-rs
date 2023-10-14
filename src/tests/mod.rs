#[cfg(test)]
mod tests {
    use crate::{
        models::types::chat::ChatId,
        telegram_bot::{TelegramBot, TelegramBotService},
    };

    #[tokio::test]
    async fn authenticate_failed_token() {
        let result = TelegramBot::new(String::from("")).authenticate().await;
        match result {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[tokio::test]
    async fn authenticate_succeed() {
        dotenv::dotenv().ok();
        let api = std::env::var("TELEGRAM_BOT_API").unwrap();

        let result = TelegramBot::new(String::from(api)).authenticate().await;

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn send_message_failed_token() {
        let result = TelegramBot::new(String::from(""))
            .send_message(crate::models::types::message::SendMessage {
                chat_id: ChatId::String("@RandomFropZ".to_string()),
                text: String::from("Hello World"),
                ..Default::default()
            })
            .await;

        match result {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[tokio::test]
    async fn send_message_succeed() {
        dotenv::dotenv().ok();
        let api = std::env::var("TELEGRAM_BOT_API").unwrap();

        let result = TelegramBot::new(String::from(api))
            .send_message(crate::models::types::message::SendMessage {
                chat_id: ChatId::String("@RandomFropZ".to_string()),
                text: String::from("Hello World"),
                ..Default::default()
            })
            .await;

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
