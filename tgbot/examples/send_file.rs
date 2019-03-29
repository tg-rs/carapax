use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::{env, io::Cursor};
use tgbot::{
    handle_updates,
    methods::{EditMessageMedia, SendAnimation, SendDocument, SendPhoto, SendVideo},
    mime,
    types::{
        InlineKeyboardButton, InputFile, InputFileReader, InputMedia, InputMediaAnimation, InputMediaPhoto,
        InputMediaVideo, MessageData, Update, UpdateKind,
    },
    Api, Config, UpdateHandler, UpdateMethod,
};

struct Handler {
    api: Api,
    gif_url: String,
    photo_path: String,
    video_path: String,
    document_thumb_path: String,
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);

        macro_rules! execute {
            ($method:expr) => {
                self.api.spawn(self.api.execute($method).then(|x| {
                    log::info!("method result: {:?}\n", x);
                    Ok::<(), ()>(())
                }));
            };
        }

        if let UpdateKind::Message(message) = update.kind {
            let chat_id = message.get_chat_id();
            if let Some(reply_to) = message.reply_to {
                match reply_to.data {
                    // Set caption and keep file_id
                    MessageData::Animation(animation) => {
                        execute!(EditMessageMedia::new(
                            chat_id,
                            reply_to.id,
                            InputMedia::new(
                                InputFile::file_id(animation.file_id),
                                InputMediaAnimation::default().caption("test")
                            )
                            .unwrap()
                        ));
                    }
                    // Change document to animation
                    MessageData::Document { .. } => {
                        execute!(EditMessageMedia::new(
                            chat_id,
                            reply_to.id,
                            InputMedia::new(
                                InputFile::url(self.gif_url.clone()),
                                InputMediaAnimation::default().caption("test")
                            )
                            .unwrap()
                        ));
                    }
                    // Change photo to video
                    MessageData::Photo { .. } => {
                        execute!(EditMessageMedia::new(
                            chat_id,
                            reply_to.id,
                            InputMedia::new(InputFile::path(self.video_path.clone()), InputMediaVideo::default())
                                .unwrap()
                        ));
                    }
                    // Change video to photo
                    MessageData::Video { .. } => {
                        execute!(EditMessageMedia::new(
                            chat_id,
                            reply_to.id,
                            InputMedia::new(InputFile::path(self.photo_path.clone()), InputMediaPhoto::default())
                                .unwrap()
                        ));
                    }
                    _ => {}
                }
            } else {
                if let MessageData::Document { data, .. } = message.data {
                    // Resend document by file id (you also can send a document using URL)
                    execute!(SendDocument::new(chat_id, InputFile::file_id(data.file_id)));
                } else if let Some(text) = message.get_text() {
                    match text.data.as_str() {
                        // Send animation by URL (you also can send animation using a file_id)
                        "/gif" => execute!(SendAnimation::new(chat_id, InputFile::url(self.gif_url.clone()))),
                        "/photo" => {
                            let markup = vec![vec![InlineKeyboardButton::with_callback_data("test", "cb-data")]];
                            execute!(SendPhoto::new(chat_id, InputFile::path(self.photo_path.clone()))
                                .reply_markup(markup)
                                .unwrap())
                        }
                        "/text" => {
                            let document = Cursor::new(b"Hello World!");
                            let reader = InputFileReader::new(document).info(("hello.txt", mime::TEXT_PLAIN));
                            execute!(SendDocument::new(chat_id, InputFile::reader(reader))
                                .thumb(InputFile::path(self.document_thumb_path.clone())))
                        }
                        "/video" => execute!(SendVideo::new(chat_id, InputFile::path(self.video_path.clone()))),
                        // The same way for other file types...
                        _ => {}
                    };
                }
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let gif_url = env::var("TGRS_GIF_URL").expect("TGRS_GIF_URL is not set");
    let photo_path = env::var("TGRS_PHOTO_PATH").expect("TGRS_PHOTO_PATH is not set");
    let video_path = env::var("TGRS_VIDEO_PATH").expect("TGRS_PHOTO_PATH is not set");
    let document_thumb_path = env::var("TGRS_DOCUMENT_THUMB_PATH").expect("TGRS_DOCUMENT_THUMB_PATH is not set");
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }
    let api = Api::new(config).expect("Failed to create API");
    tokio::run(handle_updates(
        UpdateMethod::poll(api.clone()),
        Handler {
            api,
            gif_url,
            photo_path,
            video_path,
            document_thumb_path,
        },
    ));
}
