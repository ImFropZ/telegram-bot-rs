#[cfg(test)]
mod tests {
    use crate::telegram_bot::{TelegramBot, TelegramBotService};

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
        let result = TelegramBot::new(String::from(""))
        .authenticate()
        .await;

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
