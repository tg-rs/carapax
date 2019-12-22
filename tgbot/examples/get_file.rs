use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use failure::Error;
use log;
use std::{env, path::Path};
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

async fn handle_document(api: &Api, tmpdir: &Path, document: Document) -> Result<(), Error> {
    let api = api.clone();
    let path = tmpdir.join(document.file_name.clone().unwrap_or_else(|| String::from("unknown")));
    let file = api.execute(GetFile::new(document.file_id.as_str())).await?;
    let file_path = file.file_path.unwrap();
    let data = api.download_file(file_path).await?;
    println!("Name: {:?}", document.file_name);
    println!("Mime-Type: {:?}", document.mime_type);
    println!("Document size: {:?}", document.file_size);
    println!("Downloaded size: {:?}", data.len());
    let mut file = File::create(path).await?;
    file.write_all(&data).await?;
    Ok(())
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) -> Result<(), Error> {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let MessageData::Document { data, .. } = message.data {
                handle_document(&self.api, self.tmpdir.path(), data).await?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy)?;
    }
    let api = Api::new(config)?;
    let tmpdir = tempdir()?;
    log::info!("Temp dir: {}", tmpdir.path().display());
    LongPoll::new(api.clone(), Handler { api, tmpdir }).run().await;
    Ok(())
}
