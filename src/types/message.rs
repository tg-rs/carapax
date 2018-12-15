use crate::types::chat::Chat;
use crate::types::contact::Contact;
use crate::types::games::Game;
use crate::types::location::Location;
use crate::types::media::{
    Animation, Audio, Document, PhotoSize, Sticker, Venue, Video, VideoNote, Voice,
};
use crate::types::passport::PassportData;
use crate::types::payments::{Invoice, SuccessfulPayment};
use crate::types::primitive::Integer;
use crate::types::user::User;

/// This object represents a message.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: Integer,
    /// Sender, empty for messages sent to channels
    pub from: Option<User>,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// For messages forwarded from channels, information about the original channel
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<Integer>,
    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<Integer>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further
    /// reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message is an animation, information about the animation.
    /// For backward compatibility, when this field is set, the document field will also be set
    pub animation: Option<Animation>,
    /// Message is a game, information about the game.
    pub game: Option<Game>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Message is a video note, information about the video message
    pub video_note: Option<VideoNote>,
    /// Caption for the audio, document, photo, video or voice, 0-1024 characters
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
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message
    /// in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Service message: the channel has been created.
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice.
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment.
    pub successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageEntity {
    /// Type of the entity.
    pub kind: MessageEntityKind, // TODO: rename to type
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
}

/// Type of the message entity.
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageEntityKind {
    /// Bold text (bold)
    Bold,
    /// Bot command (bot_command)
    BotCommand,
    /// Cashtag (cashtag)
    Cashtag,
    /// Monowidth string (code)
    Code,
    /// E-Mail (email)
    Email,
    /// Hashtag (hashtag)
    Hashtag,
    /// Italic text (italic)
    Italic,
    /// User mention (e.g. @username) (mention)
    Mention,
    /// Phone number (phone_number)
    PhoneNumber,
    /// Monowidth block (pre)
    Pre,
    /// Clickable text URLs (text_link)
    TextLink,
    /// Mention user without username (text_mention)
    TextMention,
    /// URL (url)
    Url,
}
