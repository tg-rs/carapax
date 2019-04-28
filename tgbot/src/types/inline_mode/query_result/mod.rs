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

pub use self::{
    article::*, audio::*, cached::*, contact::*, document::*, game::*, gif::*, location::*, mpeg4_gif::*, photo::*,
    venue::*, video::*, voice::*,
};

/// Result of an inline query
#[derive(Clone, Debug, derive_more::From, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{InlineKeyboardButton, InputMessageContentText, ParseMode};

    #[test]
    fn serialize_article() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultArticle::new("id", "title", InputMessageContentText::new("text"))
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .url("URL")
                    .hide_url(true)
                    .description("desc")
                    .thumb_url("thumb-url")
                    .thumb_width(200)
                    .thumb_height(200)
            ))
            .unwrap(),
            serde_json::json!({
                "type": "article",
                "id": "id",
                "title": "title",
                "input_message_content": {"message_text": "text"},
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "url": "URL",
                "hide_url": true,
                "description": "desc",
                "thumb_url": "thumb-url",
                "thumb_width": 200,
                "thumb_height": 200
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultArticle::new(
                "id",
                "title",
                InputMessageContentText::new("text")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "article",
                "id": "id",
                "title": "title",
                "input_message_content": {"message_text": "text"}
            })
        );
    }

    #[test]
    fn serialize_audio() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultAudio::new("id", "url", "title")
                    .caption("caption")
                    .parse_mode(ParseMode::Html)
                    .performer("performer")
                    .audio_duration(100)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "id": "id",
                "audio_url": "url",
                "title": "title",
                "caption": "caption",
                "parse_mode": "HTML",
                "performer": "performer",
                "audio_duration": 100,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultAudio::new(
                "id", "url", "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "id": "id",
                "audio_url": "url",
                "title": "title"
            })
        );
    }

    #[test]
    fn serialize_cached_audio() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedAudio::new("id", "file-id")
                    .caption("test")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "id": "id",
                "audio_file_id": "file-id",
                "caption": "test",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedAudio::new(
                "id", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "id": "id",
                "audio_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_document() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedDocument::new("id", "title", "file-id")
                    .description("desc")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "id": "id",
                "title": "title",
                "document_file_id": "file-id",
                "description": "desc",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedDocument::new(
                "id", "title", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "id": "id",
                "title": "title",
                "document_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_gif() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedGif::new("id", "file-id")
                    .title("title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "gif",
                "id": "id",
                "gif_file_id": "file-id",
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedGif::new(
                "id", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "gif",
                "id": "id",
                "gif_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_mpeg4_gif() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedMpeg4Gif::new("id", "file-id")
                    .title("title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "mpeg4_gif",
                "id": "id",
                "mpeg4_file_id": "file-id",
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedMpeg4Gif::new(
                "id", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "mpeg4_gif",
                "id": "id",
                "mpeg4_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_photo() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedPhoto::new("id", "file-id")
                    .title("title")
                    .description("desc")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "id": "id",
                "photo_file_id": "file-id",
                "title": "title",
                "description": "desc",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedPhoto::new(
                "id", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "id": "id",
                "photo_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_sticker() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedSticker::new("id", "file-id")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "sticker",
                "id": "id",
                "sticker_file_id": "file-id",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedSticker::new(
                "id", "file-id"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "sticker",
                "id": "id",
                "sticker_file_id": "file-id"
            })
        );
    }

    #[test]
    fn serialize_cached_video() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedVideo::new("id", "file-id", "title")
                    .description("desc")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "id": "id",
                "video_file_id": "file-id",
                "title": "title",
                "description": "desc",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedVideo::new(
                "id", "file-id", "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "id": "id",
                "video_file_id": "file-id",
                "title": "title"
            })
        );
    }

    #[test]
    fn serialize_cached_voice() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultCachedVoice::new("id", "file-id", "title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "voice",
                "id": "id",
                "voice_file_id": "file-id",
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedVoice::new(
                "id", "file-id", "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "voice",
                "id": "id",
                "voice_file_id": "file-id",
                "title": "title"
            })
        );
    }

    #[test]
    fn serialize_contact() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultContact::new("id", "phone", "name")
                    .last_name("last name")
                    .vcard("vcard")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
                    .thumb_url("url")
                    .thumb_width(200)
                    .thumb_height(200)
            ))
            .unwrap(),
            serde_json::json!({
                "type": "contact",
                "id": "id",
                "phone_number": "phone",
                "first_name": "name",
                "last_name": "last name",
                "vcard": "vcard",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"},
                "thumb_url": "url",
                "thumb_width": 200,
                "thumb_height": 200
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultContact::new(
                "id", "phone", "name"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "contact",
                "id": "id",
                "phone_number": "phone",
                "first_name": "name"
            })
        );
    }

    #[test]
    fn serialize_document() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultDocument::new("id", "title", "url", "mime")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .description("desc")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
                    .thumb_url("thumb-url")
                    .thumb_width(200)
                    .thumb_height(200)
            ))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "id": "id",
                "title": "title",
                "document_url": "url",
                "mime_type": "mime",
                "caption": "caption",
                "parse_mode": "Markdown",
                "description": "desc",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"},
                "thumb_url": "thumb-url",
                "thumb_width": 200,
                "thumb_height": 200
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultDocument::new(
                "id", "title", "url", "mime"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "id": "id",
                "title": "title",
                "document_url": "url",
                "mime_type": "mime"
            })
        );
    }

    #[test]
    fn serialize_game() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultGame::new("id", "name")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            ))
            .unwrap(),
            serde_json::json!({
                "type": "game",
                "id": "id",
                "game_short_name": "name",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultGame::new("id", "name"))).unwrap(),
            serde_json::json!({
                "type": "game",
                "id": "id",
                "game_short_name": "name"
            })
        );
    }

    #[test]
    fn serialize_gif() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultGif::new("id", "url", "thumb-url")
                    .gif_width(200)
                    .gif_height(300)
                    .gif_duration(400)
                    .title("title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "gif",
                "id": "id",
                "gif_url": "url",
                "thumb_url": "thumb-url",
                "gif_width": 200,
                "gif_height": 300,
                "gif_duration": 400,
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultGif::new(
                "id",
                "url",
                "thumb-url"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "gif",
                "id": "id",
                "gif_url": "url",
                "thumb_url": "thumb-url"
            })
        );
    }

    #[test]
    fn serialize_location() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultLocation::new("id", 1.0, 2.0, "title")
                    .live_period(100)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
                    .thumb_url("thumb-url")
                    .thumb_width(200)
                    .thumb_height(300)
            ))
            .unwrap(),
            serde_json::json!({
                "type": "location",
                "id": "id",
                "latitude": 1.0,
                "longitude": 2.0,
                "title": "title",
                "live_period": 100,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"},
                "thumb_url": "thumb-url",
                "thumb_width": 200,
                "thumb_height": 300
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultLocation::new(
                "id", 1.0, 2.0, "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "location",
                "id": "id",
                "latitude": 1.0,
                "longitude": 2.0,
                "title": "title"
            })
        );
    }

    #[test]
    fn serialize_mpeg4_gif() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultMpeg4Gif::new("id", "url", "thumb-url")
                    .mpeg4_width(200)
                    .mpeg4_height(300)
                    .mpeg4_duration(400)
                    .title("title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "mpeg4_gif",
                "id": "id",
                "mpeg4_url": "url",
                "thumb_url": "thumb-url",
                "mpeg4_width": 200,
                "mpeg4_height": 300,
                "mpeg4_duration": 400,
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultMpeg4Gif::new(
                "id",
                "url",
                "thumb-url"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "mpeg4_gif",
                "id": "id",
                "mpeg4_url": "url",
                "thumb_url": "thumb-url"
            })
        );
    }

    #[test]
    fn serialize_photo() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultPhoto::new("id", "url", "thumb-url")
                    .photo_width(200)
                    .photo_height(300)
                    .title("title")
                    .description("desc")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "id": "id",
                "photo_url": "url",
                "thumb_url": "thumb-url",
                "photo_width": 200,
                "photo_height": 300,
                "title": "title",
                "description": "desc",
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultPhoto::new(
                "id",
                "url",
                "thumb-url"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "id": "id",
                "photo_url": "url",
                "thumb_url": "thumb-url"
            })
        );
    }

    #[test]
    fn serialize_venue() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultVenue::new("id", 1.0, 2.0, "title", "addr")
                    .foursquare_id("f-id")
                    .foursquare_type("f-type")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
                    .thumb_url("thumb-url")
                    .thumb_width(200)
                    .thumb_height(300)
            ))
            .unwrap(),
            serde_json::json!({
                "type": "venue",
                "id": "id",
                "latitude": 1.0,
                "longitude": 2.0,
                "title": "title",
                "address": "addr",
                "foursquare_id": "f-id",
                "foursquare_type": "f-type",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"},
                "thumb_url": "thumb-url",
                "thumb_width": 200,
                "thumb_height": 300
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultVenue::new(
                "id", 1.0, 2.0, "title", "addr"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "venue",
                "id": "id",
                "latitude": 1.0,
                "longitude": 2.0,
                "title": "title",
                "address": "addr"
            })
        );
    }

    #[test]
    fn serialize_video() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultVideo::new("id", "url", "mime", "thumb-url", "title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .video_width(200)
                    .video_height(300)
                    .video_duration(400)
                    .description("desc")
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "id": "id",
                "video_url": "url",
                "mime_type": "mime",
                "thumb_url": "thumb-url",
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "video_width": 200,
                "video_height": 300,
                "video_duration": 400,
                "description": "desc",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultVideo::new(
                "id",
                "url",
                "mime",
                "thumb-url",
                "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "id": "id",
                "video_url": "url",
                "mime_type": "mime",
                "thumb_url": "thumb-url",
                "title": "title"
            })
        );
    }

    #[test]
    fn serialize_voice() {
        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(
                InlineQueryResultVoice::new("id", "url", "title")
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .voice_duration(100)
                    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                    .input_message_content(InputMessageContentText::new("text"))
            ))
            .unwrap(),
            serde_json::json!({
                "type": "voice",
                "id": "id",
                "voice_url": "url",
                "title": "title",
                "caption": "caption",
                "parse_mode": "Markdown",
                "voice_duration": 100,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
                "input_message_content": {"message_text": "text"}
            })
        );

        assert_eq!(
            serde_json::to_value(InlineQueryResult::from(InlineQueryResultVoice::new(
                "id", "url", "title"
            )))
            .unwrap(),
            serde_json::json!({
                "type": "voice",
                "id": "id",
                "voice_url": "url",
                "title": "title"
            })
        );
    }
}
