use super::{
    animation::Animation,
    audio::Audio,
    chat::Chat,
    contact::Contact,
    dice::Dice,
    document::Document,
    forum_topic::{ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened},
    game::Game,
    general_forum_topic::{GeneralForumTopicHidden, GeneralForumTopicUnhidden},
    inline_keyboard::InlineKeyboardMarkup,
    location::Location,
    passport::PassportData,
    payments::{Invoice, SuccessfulPayment},
    photo_size::PhotoSize,
    poll::Poll,
    proximity_alert_triggered::ProximityAlertTriggered,
    sticker::Sticker,
    story::Story,
    user::{User, UserShared},
    venue::Venue,
    video::{
        Video, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
        VideoNote,
    },
    voice::Voice,
    web_app::WebAppData,
    write_access_allowed::WriteAccessAllowed,
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
    pub message_id: u32,
    pub message_thread_id: Option<u32>,
    pub from: User,
    pub sender_chat: Option<Chat>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<u32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<i64>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_members: Option<Vec<User>>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub user_shared: Option<UserShared>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<WriteAccessAllowed>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_edit: Option<ForumTopicEdited>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
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
    pub message_auto_delete_time: i32,
}
