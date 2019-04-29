use dotenv::dotenv;
use mockito::{mock, server_url};
use tgbot::prelude::*;
use tokio::runtime::current_thread::block_on_all;

#[test]
fn poll() {
    dotenv().ok();
    env_logger::init();
    let _m = mock("GET", "/file/bottoken/file-path").with_body(b"file-data").create();
    let api = Api::new(Config::new("token").host(server_url())).unwrap();
    let f = api.download_file("file-path");
    let data = block_on_all(f).unwrap();
    print!("{:?}", data);
    assert_eq!(data, b"file-data");
}
