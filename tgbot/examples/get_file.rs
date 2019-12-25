use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use log;
use std::{convert::Infallible, env, path::Path};
use tempfile::{tempdir, TempDir};
use tgbot::{
    longpoll::LongPoll,
    methods::GetFile,
    types::{Document, MessageData, Update, UpdateKind},
    Api, Config, UpdateHandler,
};
use tokio::{fs::File, io::AsyncWriteExt};

struct Handler {
    api: Api,
    tmpdir: TempDir,
}

async fn handle_document(api: &Api, tmpdir: &Path, document: Document) {
    let api = api.clone();
    let path = tmpdir.join(document.file_name.clone().unwrap_or_else(|| String::from("unknown")));
    let file = api.execute(GetFile::new(document.file_id.as_str())).await.unwrap();
    let file_path = file.file_path.unwrap();
    let data = api.download_file(file_path).await.unwrap();
    println!("Name: {:?}", document.file_name);
    println!("Mime-Type: {:?}", document.mime_type);
    println!("Document size: {:?}", document.file_size);
    println!("Downloaded size: {:?}", data.len());
    let mut file = File::create(path).await.unwrap();
    file.write_all(&data).await.unwrap();
}

#[async_trait]
impl UpdateHandler for Handler {
    type Error = Infallible;

    async fn handle(&mut self, update: Update) -> Result<(), Self::Error> {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let MessageData::Document { data, .. } = message.data {
                handle_document(&self.api, self.tmpdir.path(), data).await;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temporary directory");
    log::info!("Temp dir: {}", tmpdir.path().display());
    LongPoll::new(api.clone(), Handler { api, tmpdir }).run().await;
}
