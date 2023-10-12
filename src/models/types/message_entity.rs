use super::user::User;

enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
}

pub struct MessageEntity {
    r#type: MessageEntityType,
    offset: i32,
    length: i32,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}
