use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TgResponse<T> {
    /// If  equals true, the request was successful and the result
    /// of the query can be found in the ‘result’ field
    pub ok: bool,

    /// Response object
    pub result: Option<T>,

    /// Human-readable description of the result
    pub description: Option<String>,

    /// Http error code
    pub error_code: Option<i32>,

    /// This field can help to automatically handle the error
    pub parameters: Option<ResponseParameters>,
}


/// Contains information about why a request was unsuccessful.
#[derive(Deserialize, Debug, Clone)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier
    migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated
    retry_after: Option<i64>,
}


/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update
#[derive(Deserialize, Debug, Clone)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number
    /// and increase sequentially. This ID becomes especially handy if you’re using [Webhooks],
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates for at least a week,
    /// then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,

    /// New incoming message of any kind — text, photo, sticker, etc
    pub message: Option<Message>,

    /// New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,

    /// New incoming channel post of any kind — text, photo, sticker, etc
    pub channel_post: Option<Message>,

    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,

    /// New incoming callback query
    pub callback_query: Option<CallbackQuery>,
}


/// This object represents a message
#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,

    /// Sender, empty for messages sent to channels
    pub from: Option<User>,

    /// Date the message was sent in Unix time
    pub date: i64,

    /// Conversation the message belongs to
    pub chat: Chat,

    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,

    /// For messages forwarded from channels, information about the original channel
    pub forward_from_chat: Option<Chat>,

    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,

    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,

    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i64>,

    /// For replies, the original message. Note that the Message object in this field will
    /// not contain further reply_to_message fields even if it itself is a reply
    pub reply_to_message: Option<Box<Message>>,

    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,

    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,

    /// Signature of the post author for messages in channels
    pub author_signature: Option<String>,

    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters
    pub text: Option<String>,

    /// For text messages, special entities like usernames, URLs, bot commands,
    /// etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,

    /// For messages with a caption, special entities like usernames, URLs, bot commands,
    /// etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,

    /// Message is a general file, information about the file
    pub document: Option<Document>,

    /// Message is an animation, information about the animation.
    /// For backward compatibility, when this field is set, the document field will also be set
    pub animation: Option<Animation>,

    /// Message is a game, information about the game.
    /// [More about games](https://core.telegram.org/bots/api#games)
    pub game: Option<Game>,

    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,

    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,

    /// Message is a video, information about the video
    pub video: Option<Video>,

    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,

    /// Message is a [video note](https://telegram.org/blog/video-messages-and-telescope),
    /// information about the video message
    pub video_note: Option<VideoNote>,

    /// Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    pub caption: Option<String>,

    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,

    /// Message is a shared location, information about the location
    pub location: Option<Location>,

    /// Message is a venue, information about the venue
    pub venue: Option<Venue>,

    /// New members that were added to the group or supergroup and information about them
    /// (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,

    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,

    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,

    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    /// Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,

    /// Service message: the group has been created
    pub group_chat_created: Option<bool>,

    /// Service message: the supergroup has been created.
    /// This field can‘t be received in a message coming through updates, because bot can’t be a
    /// member of a supergroup when it is created. It can only be found in reply_to_message if
    /// someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,

    /// Service message: the channel has been created. This field can‘t be received in a message
    /// coming through updates, because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel
    pub channel_chat_created: Option<bool>,

    /// The group has been migrated to a supergroup with the specified identifier
    pub migrate_to_chat_id: Option<i64>,

    /// The supergroup has been migrated from a group with the specified identifier
    pub migrate_from_chat_id: Option<i64>,

    /// Specified message was pinned. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it is itself a reply
    pub pinned_message: Option<Box<Message>>,

    /// Message is an invoice for a [payment], information about the invoice.
    /// [More about payments](https://core.telegram.org/bots/api#payments)
    pub invoice: Option<Invoice>,

    /// Message is a service message about a successful payment, information about the payment.
    /// [More about payments](https://core.telegram.org/bots/api#payments)
    pub successful_payment: Option<SuccessfulPayment>,

    /// The domain name of the website on which the user has logged in.
    /// [More about Telegram Login](https://core.telegram.org/widgets/login)
    pub connected_website: Option<String>,

    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
}


///This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc
#[derive(Deserialize, Debug, Clone)]
pub struct MessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, cashtag,
    /// bot_command, url, email, phone_number, bold (bold text), italic (italic text),
    /// code (monowidth string), pre (monowidth block), text_link (for clickable text URLs),
    /// text_mention ([for users without usernames](https://telegram.org/blog/edit#new-mentions))
    #[serde(rename = "type")]
    pub typ: String,

    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,

    /// Length of the entity in UTF-16 code units
    pub length: i64,

    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,

    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
}

#[derive(Clone, Debug)]
pub enum MessageEntityValue<'a> {
    Mention(String),
    Hashtag(String),
    Cashtag(String),
    BotCommand(String),
    Url(String),
    Email(String),
    PhoneNumber(String),
    Bold(String),
    Italic(String),
    Code(String),
    Pre(String),
    TextLink { text: String, link: &'a String },
    TextMention { mention: String, user: &'a User },
}

impl MessageEntity {
    /// Try to extract correct messageEntity from text message.
    pub fn extract_value(&self, text: &str) -> Option<MessageEntityValue> {
        let utf16_capture: Vec<u16> = text.encode_utf16().skip(self.offset as usize).take(self.length as usize).collect();
        let captured = String::from_utf16_lossy(&utf16_capture);
        match self.typ.as_ref() {
            "mention" =>
                Some(MessageEntityValue::Mention(captured)),
            "hashtag" =>
                Some(MessageEntityValue::Hashtag(captured)),
            "cashtag" =>
                Some(MessageEntityValue::Cashtag(captured)),
            "bot_command" =>
                Some(MessageEntityValue::BotCommand(captured)),
            "url" =>
                Some(MessageEntityValue::Url(captured)),
            "email" =>
                Some(MessageEntityValue::Email(captured)),
            "phone_number" =>
                Some(MessageEntityValue::PhoneNumber(captured)),
            "bold" =>
                Some(MessageEntityValue::Bold(captured)),
            "italic" =>
                Some(MessageEntityValue::Italic(captured)),
            "code" =>
                Some(MessageEntityValue::Code(captured)),
            "pre" =>
                Some(MessageEntityValue::Pre(captured)),
            "text_link" =>
                self.url.as_ref().map(|link| MessageEntityValue::TextLink { text: captured, link }),
            "text_mention" =>
                self.user.as_ref().map(|user| MessageEntityValue::TextMention { mention: captured, user }),
            _ =>
                None
        }
    }
}


/// This object represents an audio file to be treated as music by the Telegram clients
#[derive(Deserialize, Debug, Clone)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: String,

    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,

    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,

    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,

    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,

    /// File size
    pub file_size: Option<i64>,

    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}


/// This object represents a general file (as opposed to
/// [photos](https://core.telegram.org/bots/api#photosize),
/// [voice messages](https://core.telegram.org/bots/api#voice) and
/// [audio files](https://core.telegram.org/bots/api#audio))
#[derive(Deserialize, Debug, Clone)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,

    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound)
#[derive(Deserialize, Debug, Clone)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,

    /// Video width as defined by sender
    pub width: i64,

    /// Video height as defined by sender
    pub height: i64,

    /// Duration of the video in seconds as defined by sender
    pub duration: i64,

    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,

    /// Original animation filename as defined by sender
    pub file_name: Option<String>,

    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents a game. Use BotFather to create and edit games,
/// their short names will act as unique identifiers
#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    /// Title of the game
    pub title: String,

    /// Description of the game
    pub description: String,

    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    /// Brief description of the game or high scores included in the game message.
    /// Can be automatically edited to include current high scores for the game when the bot
    /// calls [set_game_score], or manually edited using
    /// [edit_message_text]. 0-4096 characters
    pub text: Option<String>,

    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc
    pub text_entities: Option<MessageEntity>,

    /// Animation that will be displayed in the game message in chats. Upload via
    /// [BotFather](https://t.me/botfather)
    pub animation: Option<Animation>,
}


/// This object represents one size of a photo or a
/// [file](https://core.telegram.org/bots/api#document) /
/// [sticker](https://core.telegram.org/bots/api#sticker) thumbnail
#[derive(Deserialize, Debug, Clone)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,

    /// Photo width
    pub width: i64,

    /// Photo height
    pub height: i64,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents a sticker
#[derive(Deserialize, Debug, Clone)]
pub struct Sticker {
    /// Unique identifier for this file
    pub file_id: String,

    /// Sticker width
    pub width: i64,

    /// Sticker height
    pub height: i64,

    /// Sticker thumbnail in the .webp or .jpg format
    pub thumb: Option<PhotoSize>,

    /// Emoji associated with the sticker
    pub emoji: Option<String>,

    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,

    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,

    /// File size
    pub file_size: Option<i64>,
}

/// This object describes the position on faces where a mask should be placed by default
#[derive(Deserialize, Debug, Clone)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed.
    /// One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,

    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right.
    /// For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f32,

    /// Shift by Y-axis measured in heights of the mask scaled to the face size,
    /// from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f32,

    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f32,
}


/// This object represents a video file
#[derive(Deserialize, Debug, Clone)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,

    /// Video width as defined by sender
    pub width: i64,

    /// Video height as defined by sender
    pub height: i64,

    /// Duration of the video in seconds as defined by sender
    pub duration: i64,

    /// Video thumbnail
    pub thumb: Option<PhotoSize>,

    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents a voice note
#[derive(Deserialize, Debug, Clone)]
pub struct Voice {
    /// Unique identifier for this file
    pub file_id: String,

    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,

    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope)
/// (available in Telegram apps as of [v.4.0](https://telegram.org/blog/video-messages-and-telescope)
#[derive(Deserialize, Debug, Clone)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,

    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,

    /// Duration of the video in seconds as defined by sender
    pub duration: i64,

    /// Video thumbnail
    pub thumb: Option<PhotoSize>,

    /// File size
    pub file_size: Option<i64>,
}


/// This object represents a phone contact
#[derive(Deserialize, Debug, Clone)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,

    /// Contact's first name
    pub first_name: String,

    /// Contact's last name
    pub last_name: Option<String>,

    /// Contact's user identifier in Telegram
    pub user_d: Option<i64>,

    /// Additional data about the contact in the
    /// form of a [vCard](https://en.wikipedia.org/wiki/VCard)
    pub vcard: Option<String>,
}


/// This object represents a point on the map
#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f32,
    /// Latitude as defined by sender
    pub latitude: f32,
}


/// This object represents a venue
#[derive(Deserialize, Debug, Clone)]
pub struct Venue {
    /// Venue location
    pub location: Location,

    /// Name of the venue
    pub title: String,

    /// Address of the venue
    pub address: String,

    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Invoice {}

#[derive(Deserialize, Debug, Clone)]
pub struct SuccessfulPayment {}

#[derive(Deserialize, Debug, Clone)]
pub struct PassportData {}

/// This object represents a file ready to be downloaded. The file can be downloaded via method
/// `download_file`
#[derive(Deserialize, Debug, Clone)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,

    /// File size, if known
    pub file_size: Option<i64>,

    /// File path. Pass it to `download_file` method to download it
    pub file_path: Option<String>,
}


/// This object represents a chat
#[derive(Deserialize, Debug, Clone)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,

    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub typ: String,

    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,

    /// First name of the other party in a private chat
    pub first_name: Option<String>,

    /// Last name of the other party in a private chat
    pub last_name: Option<String>,

    /// True if a group has ‘All Members Are Admins’ enabled
    pub all_members_are_administrators: Option<bool>,

    /// Chat photo. Returned only in [get_chat]
    pub photo: Option<ChatPhoto>,

    /// Description, for supergroups and channel chats. Returned only in [get_chat]
    pub description: Option<String>,

    /// Chat invite link, for supergroups and channel chats. Each administrator in a chat generates
    /// their own invite links, so the bot must first generate the link using [export_chat_invite_link].
    /// Returned only in [get_chat]
    pub invite_link: Option<String>,

    /// Pinned message, for supergroups and channel chats. Returned only in [get_chat]
    pub pinned_message: Option<Box<Message>>,

    /// For supergroups, name of group sticker set. Returned only in [get_chat]
    pub sticker_set_name: Option<String>,

    /// True, if the bot can change the group sticker set. Returned only in [get_chat]
    pub can_set_sticker_set: Option<bool>,
}

/// This object represents a chat photo.
#[derive(Deserialize, Debug, Clone)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo. This file_id can be used only
    /// for photo download
    pub small_file_id: String,

    /// Unique file identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download
    pub big_file_id: String,
}


/// This object represents a shipping address
#[derive(Deserialize, Debug, Clone)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,

    /// State, if applicable
    pub state: String,

    /// City
    pub city: String,

    /// First line for the address
    pub street_line1: String,

    /// Second line for the address
    pub street_line2: String,

    /// Address post code
    pub post_code: String,
}


/// This object represents information about an order
#[derive(Deserialize, Debug, Clone)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,

    /// User's phone number
    pub phone_number: Option<String>,

    /// User email
    pub email: Option<String>,

    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}


/// This object represents a Telegram user or bot.
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: i64,

    /// True, if this user is a bot
    pub is_bot: bool,

    /// User‘s or bot’s first name
    pub first_name: String,

    /// User‘s or bot’s last name
    pub last_name: Option<String>,

    /// User‘s or bot’s username
    pub username: Option<String>,

    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
}


/// This object represents an incoming callback query from a callback button in an
/// [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present. If the button was attached to a message sent via the bot
/// (in
/// [inline mode](https://core.telegram.org/bots/api#inline-mode)), the field inline_message_id will be present.
/// Exactly one of the fields data or game_short_name will be present
#[derive(Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,

    /// Sender
    pub from: User,

    /// Message with the callback button that originated the query. Note that message content
    /// and message date will not be available if the message is too old
    pub message: Option<Message>,

    /// Identifier of the message sent via the bot in inline mode, that originated the query
    pub inline_message_id: Option<String>,

    /// Global identifier, uniquely corresponding to the chat to which the message with the
    /// callback button was sent. Useful for high scores in
    /// [games](https://core.telegram.org/bots/api#games)
    pub chat_instance: String,

    /// Data associated with the callback button. Be aware that a bad client can send
    /// arbitrary data in this field
    pub data: Option<String>,

    /// Short name of a Game to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}
