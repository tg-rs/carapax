use crate::types::animation::Animation;
use crate::types::audio::Audio;
use crate::types::contact::Contact;
use crate::types::document::Document;
use crate::types::games::Game;
use crate::types::location::Location;
use crate::types::message::{Message, Text};
use crate::types::passport::PassportData;
use crate::types::payments::{Invoice, SuccessfulPayment};
use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;
use crate::types::stickers::Sticker;
use crate::types::user::User;
use crate::types::venue::Venue;
use crate::types::video::Video;
use crate::types::video_note::VideoNote;
use crate::types::voice::Voice;

/// Contains message data
#[derive(Clone, Debug)]
pub enum MessageData {
    /// Message is an animation, information about the animation
    Animation(Animation),
    /// Audio message
    Audio {
        /// Audio caption
        caption: Option<Text>,
        /// Audio data
        data: Audio,
    },
    /// Service message: the channel has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel
    ChannelChatCreated,
    /// The domain name of the website on which the user has logged in
    ConnectedWebsite(String),
    /// Message is a shared contact, information about the contact
    Contact(Contact),
    /// Service message: the chat photo was deleted
    DeleteChatPhoto,
    /// Document message
    Document {
        /// Document caption
        caption: Option<Text>,
        /// Document data
        data: Document,
    },
    /// Message is a game, information about the game
    Game(Game),
    /// Service message: the group has been created
    GroupChatCreated,
    /// Message is an invoice for a payment, information about the invoice
    Invoice(Invoice),
    /// A member was removed from the group
    /// (this member may be the bot itself)
    LeftChatMember(User),
    /// Message is a shared location, information about the location
    Location(Location),
    /// The supergroup has been migrated from a group with the specified identifier
    MigrateFromChatId(Integer),
    /// The group has been migrated to a supergroup with the specified identifier
    MigrateToChatId(Integer),
    /// New members that were added to the group or supergroup
    /// (the bot itself may be one of these members)
    NewChatMembers(Vec<User>),
    /// A chat photo was change to this value
    NewChatPhoto(Vec<PhotoSize>),
    /// A chat title was changed to this value
    NewChatTitle(String),
    /// Telegram Passport data
    PassportData(PassportData),
    /// Specified message was pinned
    /// Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply
    PinnedMessage(Box<Message>),
    /// Message is a photo, available sizes of the photo
    Photo {
        /// Photo caption
        caption: Option<Text>,
        /// Photos
        data: Vec<PhotoSize>,
    },
    /// Message is a sticker, information about the sticker
    Sticker(Sticker),
    /// Message is a service message about a successful payment, information about the payment
    SuccessfulPayment(SuccessfulPayment),
    /// Service message: the supergroup has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message
    /// in a directly created supergroup
    SupergroupChatCreated,
    /// The actual UTF-8 text of the message, 0-4096 characters
    Text(Text),
    /// Message is a venue, information about the venue
    Venue(Venue),
    /// Message is a video, information about the video
    Video {
        /// Video caption
        caption: Option<Text>,
        /// Video data
        data: Video,
    },
    /// Message is a video note, information about the video message
    VideoNote(VideoNote),
    /// Message is a voice message, information about the file
    Voice {
        /// Voice caption
        caption: Option<Text>,
        /// Voice data
        data: Voice,
    },
}
