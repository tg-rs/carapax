use dotenv::dotenv;
use env_logger;
use failure::Error;
use futures::Future;
use log;
use std::env;
use tgbot::{
    handle_updates,
    methods::GetFile,
    types::{Document, MessageData, Update, UpdateKind},
    Api, Config, UpdateHandler, UpdateMethod,
};
use tokio::fs::File;
use tokio::io::AsyncWrite;

struct Handler {
    api: Api,
}

fn handle_document(api: &Api, document: Document) -> Box<Future<Item = (), Error = Error> + Send> {
    let api = api.clone();
    let method = GetFile::new(document.file_id.as_str());
    Box::new(api.execute(&method).and_then(move |file| {
        let file_path = file.file_path.unwrap();
        api.download_file(file_path).and_then(move |data| {
            println!("Name: {:?}", document.file_name);
            println!("Mime-Type: {:?}", document.mime_type);
            println!("Document size: {:?}", document.file_size);
            println!("Downloaded size: {:?}", data.len());
            File::create(format!(
                "/tmp/tgbot-get-file-{}",
                document.file_name.unwrap_or_else(|| String::from("unknown"))
            ))
            .and_then(move |mut file| file.poll_write(&data))
            .map(|_| ())
            .from_err()
        })
    }))
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let MessageData::Document { data, .. } = message.data {
                self.api.spawn(handle_document(&self.api, data));
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }
    let api = Api::new(config).expect("Failed to create API");
    tokio::run(handle_updates(UpdateMethod::poll(api.clone()), Handler { api }));
}
