use super::chat::ChatId;

pub struct BotName {
    pub name: String,
}

pub struct BotDescription {
    pub description: String,
}

pub struct BotShortDescription {
    pub short_description: String,
}

pub enum BotCommandScope {
    Default {
        r#type: String,
    },
    AllPrivateChats {
        r#type: String,
    },
    AllGroupChats {
        r#type: String,
    },
    AllChatAdministrators {
        r#type: String,
    },
    Chat {
        r#type: String,
        chat_id: ChatId,
    },
    ChatAdministrators {
        r#type: String,
        chat_id: ChatId,
    },
    ChatMember {
        r#type: String,
        chat_id: ChatId,
        user_id: i64,
    },
}

pub struct BotCommand {
    pub command: String,
    pub description: String,
}
