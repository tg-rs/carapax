use dotenv::dotenv;
use env_logger;
use failure::Error;
use futures::{future, sync::oneshot::channel, Future, Stream};
use hyper::{header::HeaderValue, Body, Client, Method, Request, Server, StatusCode};
use log;
use tgbot::prelude::*;
use tokio::runtime::current_thread::block_on_all;

struct Handler;

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::debug!("got an update: {:?}\n", update);
    }
}

#[test]
fn webhook() {
    dotenv().ok();
    env_logger::init();
    let (tx, rx) = channel::<()>();
    let server = Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(WebhookServiceFactory::new("/", Handler))
        .with_graceful_shutdown(rx)
        .map_err(|e| log::error!("Server error: {}", e));
    let (status, body) = block_on_all(future::lazy(|| {
        tokio::spawn(server);
        let client = Client::new();
        let json = r#"{
            "update_id":10000,
            "message":{
                "date":1441645532,
                "chat":{
                    "last_name":"Test Lastname",
                    "id":1111111,
                    "first_name":"Test",
                    "username":"Test",
                    "type": "private"
                },
                "message_id":1365,
                "from":{
                    "last_name":"Test Lastname",
                    "id":1111111,
                    "first_name":"Test",
                    "username":"Test",
                    "is_bot": false
                },
                "text":"/start"
            }
        }"#;
        let uri: hyper::Uri = "http://localhost:8080/".parse().unwrap();
        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        client.request(req).map_err(Error::from).and_then(|res| {
            let _ = tx.send(());
            let status = res.status();
            res.into_body()
                .concat2()
                .map_err(Error::from)
                .and_then(|body| String::from_utf8(body.into_iter().collect()).map_err(Error::from))
                .map(move |body| (status, body))
        })
    }))
    .unwrap();
    log::debug!("Webhook response body: {:?}", body);
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "");
}
