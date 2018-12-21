use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::{Float, Integer, ParseMode};
use crate::types::reply_markup::InlineKeyboardMarkup;

/// Result of an inline query
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum InlineQueryResult {
    /// Link to an article or web page
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),
    /// Link to an mp3 audio file
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),
    /// Link to an mp3 audio file stored on the Telegram servers
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    /// Link to a file stored on the Telegram servers
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    /// Link to an animated GIF file stored on the Telegram servers
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    /// Link to a video animation
    /// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// Link to a photo stored on the Telegram servers
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// Link to a sticker stored on the Telegram servers
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    /// Link to a video file stored on the Telegram servers
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    /// Link to a voice message stored on the Telegram servers
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),
    /// Contact with a phone number
    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),
    /// Link to a file
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),
    /// Game
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),
    /// Link to an animated GIF file
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),
    /// Location on a map
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),
    /// Link to a video animation (H.264/MPEG-4 AVC video without sound)
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// Link to a photo
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),
    /// Venue
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),
    /// Link to a page containing an embedded video player or a video file
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),
    /// Link to a voice recording in an .ogg container encoded with OPUS
    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

macro_rules! impl_query_result_from {
    ($to:ident($from:ident)) => {
        impl From<$from> for InlineQueryResult {
            fn from(obj: $from) -> InlineQueryResult {
                InlineQueryResult::$to(obj)
            }
        }
    };
}

impl_query_result_from!(Article(InlineQueryResultArticle));
impl_query_result_from!(Audio(InlineQueryResultAudio));
impl_query_result_from!(CachedAudio(InlineQueryResultCachedAudio));
impl_query_result_from!(CachedDocument(InlineQueryResultCachedDocument));
impl_query_result_from!(CachedGif(InlineQueryResultCachedGif));
impl_query_result_from!(CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif));
impl_query_result_from!(CachedPhoto(InlineQueryResultCachedPhoto));
impl_query_result_from!(CachedSticker(InlineQueryResultCachedSticker));
impl_query_result_from!(CachedVideo(InlineQueryResultCachedVideo));
impl_query_result_from!(CachedVoice(InlineQueryResultCachedVoice));
impl_query_result_from!(Contact(InlineQueryResultContact));
impl_query_result_from!(Document(InlineQueryResultDocument));
impl_query_result_from!(Game(InlineQueryResultGame));
impl_query_result_from!(Gif(InlineQueryResultGif));
impl_query_result_from!(Location(InlineQueryResultLocation));
impl_query_result_from!(Mpeg4Gif(InlineQueryResultMpeg4Gif));
impl_query_result_from!(Photo(InlineQueryResultPhoto));
impl_query_result_from!(Venue(InlineQueryResultVenue));
impl_query_result_from!(Video(InlineQueryResultVideo));
impl_query_result_from!(Voice(InlineQueryResultVoice));

/// Link to an article or web page
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    pub url: Option<String>,
    /// Pass True, if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Short description of the result
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

impl InlineQueryResultArticle {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(
        id: S,
        title: S,
        input_message_content: InputMessageContent,
    ) -> Self {
        InlineQueryResultArticle {
            id: id.into(),
            title: title.into(),
            input_message_content,
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Link to an mp3 audio file
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the audio
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Performer
    pub performer: Option<String>,
    /// Audio duration in seconds
    pub audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultAudio {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, audio_url: S, title: S) -> Self {
        InlineQueryResultAudio {
            id: id.into(),
            audio_url: audio_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to an mp3 audio file stored on the Telegram servers
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the audio
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedAudio {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, audio_file_id: S) -> Self {
        InlineQueryResultCachedAudio {
            id: id.into(),
            audio_file_id: audio_file_id.into(),
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a file stored on the Telegram servers
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the file
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedDocument {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, title: S, document_file_id: S) -> Self {
        InlineQueryResultCachedDocument {
            id: id.into(),
            title: title.into(),
            document_file_id: document_file_id.into(),
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to an animated GIF file stored on the Telegram servers
///
/// By default, this animated GIF file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send
/// a message with specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedGif {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, gif_file_id: S) -> Self {
        InlineQueryResultCachedGif {
            id: id.into(),
            gif_file_id: gif_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
///
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MP4 file
    pub mpeg4_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedMpeg4Gif {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, mpeg4_file_id: S) -> Self {
        InlineQueryResultCachedMpeg4Gif {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a photo stored on the Telegram servers
///
/// By default, this photo will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the photo
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedPhoto {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, photo_file_id: S) -> Self {
        InlineQueryResultCachedPhoto {
            id: id.into(),
            photo_file_id: photo_file_id.into(),
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a sticker stored on the Telegram servers
///
/// By default, this sticker will be sent by the user
/// Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the sticker
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedSticker {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, sticker_file_id: S) -> Self {
        InlineQueryResultCachedSticker {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a video file stored on the Telegram servers
///
/// By default, this video file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the video
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedVideo {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, video_file_id: S, title: S) -> Self {
        InlineQueryResultCachedVideo {
            id: id.into(),
            video_file_id: video_file_id.into(),
            title: title.into(),
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a voice message stored on the Telegram servers
///
/// By default, this voice message will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the voice message
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedVoice {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, voice_file_id: S, title: S) -> Self {
        InlineQueryResultCachedVoice {
            id: id.into(),
            voice_file_id: voice_file_id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Contact with a phone number
///
/// By default, this contact will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the contact
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

impl InlineQueryResultContact {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, phone_number: S, first_name: S) -> Self {
        InlineQueryResultContact {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Link to a file
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the file
/// Currently, only .PDF and .ZIP files can be sent using this method
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail (jpeg only) for the file
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

impl InlineQueryResultDocument {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, title: S, document_url: S, mime_type: S) -> Self {
        InlineQueryResultDocument {
            id: id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            document_url: document_url.into(),
            mime_type: mime_type.into(),
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Game
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, game_short_name: S) -> Self {
        InlineQueryResultGame {
            id: id.into(),
            game_short_name: game_short_name.into(),
            reply_markup: None,
        }
    }
}

/// Link to an animated GIF file
///
/// By default, this animated GIF file
/// will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Width of the GIF
    pub gif_width: Option<Integer>,
    /// Height of the GIF
    pub gif_height: Option<Integer>,
    /// Duration of the GIF
    pub gif_duration: Option<Integer>,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultGif {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, gif_url: S, thumb_url: S) -> Self {
        InlineQueryResultGif {
            id: id.into(),
            gif_url: gif_url.into(),
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_url: thumb_url.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Location on a map
///
/// By default, the location will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the location
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultLocation {
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
}

impl InlineQueryResultLocation {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, latitude: Float, longitude: Float, title: S) -> Self {
        InlineQueryResultLocation {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            live_period: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Link to a video animation (H.264/MPEG-4 AVC video without sound)
///
/// By default, this animated MPEG-4 file will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Video width
    pub mpeg4_width: Option<Integer>,
    /// Video height
    pub mpeg4_height: Option<Integer>,
    /// Video duration
    pub mpeg4_duration: Option<Integer>,
    /// URL of the static thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultMpeg4Gif {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, mpeg4_url: S, thumb_url: S) -> Self {
        InlineQueryResultMpeg4Gif {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url: thumb_url.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a photo
///
/// By default, this photo will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the photo
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo.
    /// Photo must be in jpeg format.
    /// Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    ///  Width of the photo
    pub photo_width: Option<Integer>,
    /// Height of the photo
    pub photo_height: Option<Integer>,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultPhoto {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, photo_url: S, thumb_url: S) -> Self {
        InlineQueryResultPhoto {
            id: id.into(),
            photo_url: photo_url.into(),
            thumb_url: thumb_url.into(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Venue
///
/// By default, the venue will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the venue
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: Float,
    /// Longitude of the venue location in degrees
    pub longitude: Float,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

impl InlineQueryResultVenue {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(
        id: S,
        latitude: Float,
        longitude: Float,
        title: S,
        address: S,
    ) -> Self {
        InlineQueryResultVenue {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Link to a page containing an embedded video player or a video file
///
/// By default, this video file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the video
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
/// you must replace its content using input_message_content
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Video width
    pub video_width: Option<Integer>,
    /// Video height
    pub video_height: Option<Integer>,
    /// Video duration in seconds
    pub video_duration: Option<Integer>,
    /// Short description of the result
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video.
    /// This field is required if InlineQueryResultVideo
    /// is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, video_url: S, mime_type: S, thumb_url: S, title: S) -> Self {
        InlineQueryResultVideo {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumb_url: thumb_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Link to a voice recording in an .ogg container encoded with OPUS
///
/// By default, this voice recording will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the the voice message
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Recording duration in seconds
    pub voice_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVoice {
    /// Returns a new query result with empty optional parameters
    pub fn new<S: Into<String>>(id: S, voice_url: S, title: S) -> Self {
        InlineQueryResultVoice {
            id: id.into(),
            voice_url: voice_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
