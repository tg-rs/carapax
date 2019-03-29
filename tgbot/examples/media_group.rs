use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;
use tgbot::{
    handle_updates,
    methods::SendMediaGroup,
    types::{InputFile, MediaGroup, Update},
    Api, Config, UpdateHandler, UpdateMethod,
};

struct Handler {
    api: Api,
    photo_path: String,
    photo_url: String,
    video_path: String,
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let Some(chat_id) = update.get_chat_id() {
            let mut media = MediaGroup::default();
            let photo_info = media.add_photo(InputFile::url(self.photo_url.clone()));
            photo_info.caption("Photo 01");
            let photo_info = media.add_photo(InputFile::path(self.photo_path.clone()));
            photo_info.caption("Photo 02");
            let video_info = media.add_video(InputFile::path(self.video_path.clone()));
            video_info.caption("Video 01");
            let method = SendMediaGroup::new(chat_id, media).unwrap();
            self.api.spawn(self.api.execute(method).then(|x| {
                log::info!("sendMessage result: {:?}\n", x);
                Ok::<(), ()>(())
            }));
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let photo_path = env::var("TGRS_PHOTO_PATH").expect("TGRS_PHOTO_PATH is not set");
    let photo_url = env::var("TGRS_PHOTO_URL").expect("TGRS_PHOTO_URL is not set");
    let video_path = env::var("TGRS_VIDEO_PATH").expect("TGRS_VIDEO_PATH is not set");
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }
    let api = Api::new(config).expect("Failed to create API");
    tokio::run(handle_updates(
        UpdateMethod::poll(api.clone()),
        Handler {
            api,
            photo_path,
            photo_url,
            video_path,
        },
    ));
}
