use super::{
    animation::Animation,
    audio::Audio,
    chat::Chat,
    contact::Contact,
    dice::Dice,
    document::Document,
    forum_topic::{ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened},
    game::Game,
    inline_keyboard::InlineKeyboardMarkup,
    location::Location,
    photo_size::PhotoSize,
    poll::Poll,
    proximity_alert_triggered::ProximityAlertTriggered,
    story::Story,
    user::User,
    venue::Venue,
    video::Video,
    video_chat::{
        VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
    },
    video_note::VideoNote,
    voice::Voice,
    web_app_data::WebAppData,
};

pub enum MessageEntityType {
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

pub struct Message {
    message_id: u32,
    message_thread_id: Option<u32>,
    from: User,
    sender_chat: Option<Chat>,
    date: i64,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<u32>,
    forward_signature: Option<String>,
    forward_sender_name: Option<String>,
    forward_date: Option<i64>,
    is_topic_message: Option<bool>,
    is_automatic_forward: Option<bool>,
    reply_to_message: Option<Box<Message>>,
    via_bot: Option<User>,
    edit_date: Option<i64>,
    has_protected_content: Option<bool>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    story: Option<Story>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_media_spoiler: Option<bool>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    game: Option<Game>,
    poll: Option<Poll>,
    venue: Option<Venue>,
    location: Option<Location>,
    new_chat_members: Option<Vec<User>>,
    left_chat_members: Option<Vec<User>>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    user_shared: Option<UserShared>,
    connected_website: Option<String>,
    write_access_allowed: Option<WriteAccessAllowed>,
    passport_data: Option<PassportData>,
    proximity_alert_triggered: Option<ProximityAlertTriggered>,
    forum_topic_created: Option<ForumTopicCreated>,
    forum_topic_edit: Option<ForumTopicEdited>,
    forum_topic_closed: Option<ForumTopicClosed>,
    forum_topic_reopened: Option<ForumTopicReopened>,
    general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    video_chat_scheduled: Option<VideoChatScheduled>,
    video_chat_started: Option<VideoChatStarted>,
    video_chat_ended: Option<VideoChatEnded>,
    video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    web_app_data: Option<WebAppData>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

pub struct MessageEntity {
    pub r#type: MessageEntityType,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: i32,
}
