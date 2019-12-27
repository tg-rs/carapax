use async_trait::async_trait;
use bytes::Buf;
use dotenv::dotenv;
use env_logger;
use hyper::{body, header::HeaderValue, Body, Client, Method, Request, Server, StatusCode};
use std::{convert::Infallible, sync::Arc};
use tgbot::{types::Update, webhook::WebhookServiceFactory, UpdateHandler};
use tokio::sync::{oneshot::channel, Mutex};

struct Handler {
    updates: Arc<Mutex<Vec<Update>>>,
}

#[async_trait]
impl UpdateHandler for Handler {
    type Error = Infallible;

    async fn handle(&mut self, update: Update) -> Result<(), Self::Error> {
        let mut updates = self.updates.lock().await;
        updates.push(update);
        Ok(())
    }
}

#[tokio::test]
async fn webhook() {
    dotenv().ok();
    env_logger::init();
    let (tx, rx) = channel::<()>();
    let updates = Arc::new(Mutex::new(Vec::new()));
    let server = Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(WebhookServiceFactory::new(
            "/",
            Handler {
                updates: updates.clone(),
            },
        ))
        .with_graceful_shutdown(async {
            rx.await.ok();
        });
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
    let rep = client.request(req).await.unwrap();
    let _ = tx.send(());
    let status = rep.status();
    let data = body::aggregate(rep).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(data.bytes(), b"");
    assert!(!updates.lock().await.is_empty())
}
