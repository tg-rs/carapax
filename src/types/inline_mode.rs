use crate::types::keyboards::InlineKeyboardMarkup;
use crate::types::location::Location;
use crate::types::primitive::{Float, Integer, ParseMode};
use crate::types::user::User;
// use serde::ser::{Serialize, Serializer, SerializeStruct};

/// This object represents an incoming inline query.
/// When the user sends an empty query, your bot could return some default or trending results.
#[derive(Debug, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

/// This object represents one result of an inline query
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    /// Represents a link to an article or web page.
    #[serde(rename = "article")]
    Article {
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Title of the result
        title: String,
        /// Content of the message to be sent
        input_message_content: InputMessageContent,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// URL of the result
        url: Option<String>,
        /// Pass True, if you don't want the URL to be shown in the message
        hide_url: Option<bool>,
        /// Short description of the result
        description: Option<String>,
        /// Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Thumbnail width
        thumb_width: Option<Integer>,
        /// Thumbnail height
        thumb_height: Option<Integer>,
    },
    /// Represents a link to an mp3 audio file.
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use input_message_content to send
    /// a message with the specified content instead of the audio.
    #[serde(rename = "audio")]
    Audio {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the audio file
        audio_url: String,
        /// Title
        title: String,
        /// Caption, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Performer
        performer: Option<String>,
        /// Audio duration in seconds
        audio_duration: Option<Integer>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the audio
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an mp3 audio file
    /// stored on the Telegram servers.
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the audio.
    #[serde(rename = "audio")]
    CachedAudio {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the audio file
        audio_file_id: String,
        /// Caption, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the audio
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a file
    /// stored on the Telegram servers.
    /// By default, this file will be sent
    /// by the user with an optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the file.
    #[serde(rename = "document")]
    CachedDocument {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Title for the result
        title: String,
        /// A valid file identifier for the file
        document_file_id: String,
        /// Short description of the result
        description: Option<String>,
        /// Caption of the document to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the file
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an animated GIF file
    /// stored on the Telegram servers.
    /// By default, this animated GIF file will be sent
    /// by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send
    /// a message with specified content instead of the animation.
    #[serde(rename = "gif")]
    CachedGif {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the GIF file
        gif_file_id: String,
        /// Title for the result
        title: Option<String>,
        /// Caption of the GIF file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the GIF animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a video animation
    /// (H.264/MPEG-4 AVC video without sound)
    /// stored on the Telegram servers.
    /// By default, this animated MPEG-4 file
    /// will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content
    /// instead of the animation.
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the MP4 file
        mpeg4_file_id: String,
        /// Title for the result
        title: Option<String>,
        /// Caption of the MPEG-4 file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the video animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a photo stored on the Telegram servers.
    /// By default, this photo will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send
    /// a message with the specified content instead of the photo.
    #[serde(rename = "photo")]
    CachedPhoto {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier of the photo
        photo_file_id: String,
        /// Title for the result
        title: Option<String>,
        /// Short description of the result
        description: Option<String>,
        /// Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the photo
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a sticker stored on the Telegram servers.
    /// By default, this sticker will be sent by the user.
    /// Alternatively, you can use input_message_content to
    /// send a message with the specified content instead of the sticker.
    #[serde(rename = "sticker")]
    CachedSticker {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier of the sticker
        sticker_file_id: String,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the sticker
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a video file
    /// stored on the Telegram servers.
    /// By default, this video file
    /// will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the video.
    #[serde(rename = "video")]
    CachedVideo {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the video file
        video_file_id: String,
        /// Title for the result
        title: String,
        /// Short description of the result
        description: Option<String>,
        /// Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the video
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a voice message
    /// stored on the Telegram servers.
    /// By default, this voice message
    /// will be sent by the user.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content
    /// instead of the voice message.
    #[serde(rename = "voice")]
    CachedVoice {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the voice message
        voice_file_id: String,
        /// Voice message title
        title: String,
        /// Caption, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the voice message
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a contact with a phone number.
    /// By default, this contact will be sent by the user.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the contact.
    #[serde(rename = "contact")]
    Contact {
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Contact's phone number
        phone_number: String,
        /// Contact's first name
        first_name: String,
        /// Contact's last name
        last_name: Option<String>,
        /// Additional data about the contact in the form of a vCard, 0-2048 bytes
        vcard: Option<String>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the contact
        input_message_content: Option<InputMessageContent>,
        /// Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Thumbnail width
        thumb_width: Option<Integer>,
        /// Thumbnail height
        thumb_height: Option<Integer>,
    },
    /// Represents a link to a file.
    /// By default, this file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message
    /// with the specified content instead of the file.
    /// Currently, only .PDF and .ZIP files can be sent using this method.
    #[serde(rename = "document")]
    Document {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Title for the result
        title: String,
        /// Caption of the document to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// A valid URL for the file
        document_url: String,
        /// Mime type of the content of the file, either “application/pdf” or “application/zip”
        mime_type: String,
        /// Short description of the result
        description: Option<String>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the file
        input_message_content: Option<InputMessageContent>,
        /// URL of the thumbnail (jpeg only) for the file
        thumb_url: Option<String>,
        /// Thumbnail width
        thumb_width: Option<Integer>,
        /// Thumbnail height
        thumb_height: Option<Integer>,
    },
    /// Represents a Game
    #[serde(rename = "game")]
    Game {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Short name of the game
        game_short_name: String,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    /// Represents a link to an animated GIF file.
    /// By default, this animated GIF file
    /// will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the animation.
    #[serde(rename = "gif")]
    Gif {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the GIF file. File size must not exceed 1MB
        gif_url: String,
        /// Width of the GIF
        gif_width: Option<Integer>,
        /// Height of the GIF
        gif_height: Option<Integer>,
        /// Duration of the GIF
        gif_duration: Option<Integer>,
        /// URL of the static thumbnail for the result (jpeg or gif)
        thumb_url: String,
        /// Title for the result
        title: Option<String>,
        /// Caption of the GIF file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the GIF animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a location on a map.
    /// By default, the location will be sent by the user.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the location.
    #[serde(rename = "location")]
    Location {
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Location latitude in degrees
        latitude: Float,
        /// Location longitude in degrees
        longitude: Float,
        /// Location title
        title: String,
        /// Period in seconds for
        /// which the location can be updated,
        /// should be between 60 and 86400.
        live_period: Option<Integer>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the location
        input_message_content: Option<InputMessageContent>,
        /// Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Thumbnail width
        thumb_width: Option<Integer>,
        /// Thumbnail height
        thumb_height: Option<Integer>,
    },
    /// Represents a link to a video animation
    /// (H.264/MPEG-4 AVC video without sound).
    /// By default, this animated MPEG-4 file
    /// will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content
    /// instead of the animation.
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the MP4 file. File size must not exceed 1MB
        mpeg4_url: String,
        /// Video width
        mpeg4_width: Option<Integer>,
        /// Video height
        mpeg4_height: Option<Integer>,
        /// Video duration
        mpeg4_duration: Option<Integer>,
        /// URL of the static thumbnail (jpeg or gif) for the result
        thumb_url: String,
        /// Title for the result
        title: Option<String>,
        /// Caption of the MPEG-4 file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the video animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a photo.
    /// By default, this photo will be sent by the user with optional caption.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the photo.
    #[serde(rename = "photo")]
    Photo {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL of the photo.
        /// Photo must be in jpeg format.
        /// Photo size must not exceed 5MB
        photo_url: String,
        /// URL of the thumbnail for the photo
        thumb_url: String,
        ///  Width of the photo
        photo_width: Option<Integer>,
        /// Height of the photo
        photo_height: Option<Integer>,
        /// Title for the result
        title: Option<String>,
        /// Short description of the result
        description: Option<String>,
        /// Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the photo
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a venue.
    /// By default, the venue will be sent by the user.
    /// Alternatively, you can use input_message_content
    /// to send a message with the specified content
    /// instead of the venue.
    #[serde(rename = "venue")]
    Venue {
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Latitude of the venue location in degrees
        latitude: Float,
        /// Longitude of the venue location in degrees
        longitude: Float,
        /// Title of the venue
        title: String,
        /// Address of the venue
        address: String,
        /// Foursquare identifier of the venue if known
        foursquare_id: Option<String>,
        /// Foursquare type of the venue, if known.
        /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
        foursquare_type: Option<String>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the venue
        input_message_content: Option<InputMessageContent>,
        /// Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Thumbnail width
        thumb_width: Option<Integer>,
        /// Thumbnail height
        thumb_height: Option<Integer>,
    },
    /// Represents a link to a page containing an embedded video player or a video file.
    /// By default, this video file will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with
    /// the specified content instead of the video.
    /// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
    /// you must replace its content using input_message_content.
    #[serde(rename = "video")]
    Video {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the embedded video player or video file
        video_url: String,
        /// Mime type of the content of video url, “text/html” or “video/mp4”
        mime_type: String,
        /// URL of the thumbnail (jpeg only) for the video
        thumb_url: String,
        /// Title for the result
        title: String,
        /// Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Video width
        video_width: Option<Integer>,
        /// Video height
        video_height: Option<Integer>,
        /// Video duration in seconds
        video_duration: Option<Integer>,
        /// Short description of the result
        description: Option<String>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the video.
        /// This field is required if InlineQueryResultVideo
        /// is used to send an HTML-page as a result (e.g., a YouTube video).
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a voice recording in an .ogg container encoded with OPUS.
    /// By default, this voice recording will be sent by the user.
    /// Alternatively, you can use input_message_content to send
    /// a message with the specified content instead of the the voice message.
    #[serde(rename = "voice")]
    Voice {
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the voice recording
        voice_url: String,
        /// Recording title
        title: String,
        /// Caption, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Recording duration in seconds
        voice_duration: Option<Integer>,
        /// Inline keyboard attached to the message
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Content of the message to be sent instead of the voice recording
        input_message_content: Option<InputMessageContent>,
    },
}

/// This object represents the content of a message to be sent as a result of an inline query.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Represents the content of a text message to be sent as the result of an inline query.
    Text {
        /// Text of the message to be sent, 1-4096 characters
        message_text: String,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Disables link previews for links in the sent message
        disable_web_page_preview: Option<bool>,
    },
    /// Represents the content of a location message to be sent as the result of an inline query.
    Location {
        /// Latitude of the location in degrees
        latitude: Float,
        /// Longitude of the location in degrees
        longitude: Float,
        /// Period in seconds for which the location can be updated, should be between 60 and 86400.
        live_period: Option<Integer>,
    },
    /// Represents the content of a venue message to be sent as the result of an inline query.
    Venue {
        /// Latitude of the venue in degrees
        latitude: Float,
        /// Longitude of the venue in degrees
        longitude: Float,
        /// Name of the venue
        title: String,
        /// Address of the venue
        address: String,
        /// Foursquare identifier of the venue, if known
        foursquare_id: Option<String>,
        /// Foursquare type of the venue, if known.
        /// (For example, “arts_entertainment/default”,
        /// “arts_entertainment/aquarium” or “food/icecream”.)
        foursquare_type: Option<String>,
    },
    /// Represents the content of a contact message to be sent as the result of an inline query.
    Contact {
        /// Contact's phone number
        phone_number: String,
        /// Contact's first name
        first_name: String,
        /// Contact's last name
        last_name: Option<String>,
        /// Additional data about the contact in the form of a vCard, 0-2048 bytes
        vcard: Option<String>,
    },
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Debug, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message.
    /// Will be also received in callback queries and can be used to edit the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
