use crate::macros::impl_enum_from;
use serde::Serialize;

mod article;
mod audio;
mod cached;
mod contact;
mod document;
mod game;
mod gif;
mod location;
mod mpeg4_gif;
mod photo;
mod venue;
mod video;
mod voice;

pub use self::article::*;
pub use self::audio::*;
pub use self::cached::*;
pub use self::contact::*;
pub use self::document::*;
pub use self::game::*;
pub use self::gif::*;
pub use self::location::*;
pub use self::mpeg4_gif::*;
pub use self::photo::*;
pub use self::venue::*;
pub use self::video::*;
pub use self::voice::*;

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

impl_enum_from!(
    InlineQueryResult {
        Article(InlineQueryResultArticle),
        Audio(InlineQueryResultAudio),
        CachedAudio(InlineQueryResultCachedAudio),
        CachedDocument(InlineQueryResultCachedDocument),
        CachedGif(InlineQueryResultCachedGif),
        CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
        CachedPhoto(InlineQueryResultCachedPhoto),
        CachedSticker(InlineQueryResultCachedSticker),
        CachedVideo(InlineQueryResultCachedVideo),
        CachedVoice(InlineQueryResultCachedVoice),
        Contact(InlineQueryResultContact),
        Document(InlineQueryResultDocument),
        Game(InlineQueryResultGame),
        Gif(InlineQueryResultGif),
        Location(InlineQueryResultLocation),
        Mpeg4Gif(InlineQueryResultMpeg4Gif),
        Photo(InlineQueryResultPhoto),
        Venue(InlineQueryResultVenue),
        Video(InlineQueryResultVideo),
        Voice(InlineQueryResultVoice)
    }
);
