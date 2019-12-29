use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use log;
use std::{env, io::Cursor};
use tgbot::{
    longpoll::LongPoll,
    methods::{EditMessageMedia, SendAnimation, SendDocument, SendPhoto, SendVideo},
    mime,
    types::{
        InlineKeyboardButton, InputFile, InputFileReader, InputMedia, InputMediaAnimation, InputMediaPhoto,
        InputMediaVideo, MessageData, Update, UpdateKind,
    },
    Api, Config, UpdateHandler,
};

struct Handler {
    api: Api,
    gif_url: String,
    photo_path: String,
    video_path: String,
    document_thumb_path: String,
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            let chat_id = message.get_chat_id();
            if let Some(reply_to) = message.reply_to {
                match reply_to.data {
                    // Change animation to document
                    MessageData::Animation(_) => {
                        let input_media = InputMedia::with_thumb(
                            InputFileReader::new(Cursor::new(b"Hello World!")).info(("hello.txt", mime::TEXT_PLAIN)),
                            InputFile::path(self.document_thumb_path.clone()),
                            InputMediaAnimation::default().caption("test"),
                        )
                        .unwrap();
                        self.api
                            .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
                            .await
                            .unwrap();
                    }
                    // Change document to animation
                    MessageData::Document { .. } => {
                        self.api
                            .execute(EditMessageMedia::new(
                                chat_id,
                                reply_to.id,
                                InputMedia::new(
                                    InputFile::url(self.gif_url.clone()),
                                    InputMediaAnimation::default().caption("test"),
                                )
                                .unwrap(),
                            ))
                            .await
                            .unwrap();
                    }
                    // Change photo to video
                    MessageData::Photo { .. } => {
                        let input_media =
                            InputMedia::new(InputFile::path(self.video_path.clone()), InputMediaVideo::default())
                                .unwrap();
                        self.api
                            .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
                            .await
                            .unwrap();
                    }
                    // Change video to photo
                    MessageData::Video { .. } => {
                        let input_media =
                            InputMedia::new(InputFile::path(self.photo_path.clone()), InputMediaPhoto::default())
                                .unwrap();
                        self.api
                            .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
                            .await
                            .unwrap();
                    }
                    _ => {}
                }
            } else if let MessageData::Document { data, .. } = message.data {
                // Resend document by file id (you also can send a document using URL)
                self.api
                    .execute(SendDocument::new(chat_id, InputFile::file_id(data.file_id)))
                    .await
                    .unwrap();
            } else if let Some(text) = message.get_text() {
                match text.data.as_str() {
                    // Send animation by URL (you also can send animation using a file_id)
                    "/gif" => {
                        let method = SendAnimation::new(chat_id, InputFile::url(self.gif_url.clone()));
                        self.api.execute(method).await.unwrap();
                    }
                    "/photo" => {
                        let markup = vec![vec![InlineKeyboardButton::with_callback_data("test", "cb-data")]];
                        let method = SendPhoto::new(chat_id, InputFile::path(self.photo_path.clone()))
                            .reply_markup(markup)
                            .unwrap();
                        self.api.execute(method).await.unwrap();
                    }
                    "/text" => {
                        let document = Cursor::new(b"Hello World!");
                        let reader = InputFileReader::new(document).info(("hello.txt", mime::TEXT_PLAIN));
                        let method =
                            SendDocument::new(chat_id, reader).thumb(InputFile::path(self.document_thumb_path.clone()));
                        self.api.execute(method).await.unwrap();
                    }
                    "/video" => {
                        let method = SendVideo::new(chat_id, InputFile::path(self.video_path.clone()));
                        self.api.execute(method).await.unwrap();
                    }
                    // The same way for other file types...
                    _ => {}
                };
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let gif_url = env::var("TGRS_GIF_URL").expect("TGRS_GIF_URL is not set");
    let photo_path = env::var("TGRS_PHOTO_PATH").expect("TGRS_PHOTO_PATH is not set");
    let video_path = env::var("TGRS_VIDEO_PATH").expect("TGRS_VIDEO_PATH is not set");
    let document_thumb_path = env::var("TGRS_DOCUMENT_THUMB_PATH").expect("TGRS_DOCUMENT_THUMB_PATH is not set");
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(
        api.clone(),
        Handler {
            api,
            gif_url,
            photo_path,
            video_path,
            document_thumb_path,
        },
    )
    .run()
    .await;
}
