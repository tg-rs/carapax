use dotenv::dotenv;
use mockito::{mock, server_url};
use tgbot::prelude::*;

#[tokio::test]
async fn download_file() {
    dotenv().ok();
    env_logger::init();
    let _m = mock("GET", "/file/bottoken/file-path").with_body(b"file-data").create();
    let api = Api::new(Config::new("token").host(server_url())).unwrap();
    let data = api.download_file("file-path").await.unwrap();
    assert_eq!(&data[..], b"file-data");
}
