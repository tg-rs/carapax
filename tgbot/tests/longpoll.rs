use async_trait::async_trait;
use dotenv::dotenv;
use mockito::{mock, server_url, Matcher};
use serde_json::json;
use std::{
    convert::Infallible,
    sync::Arc,
    time::{Duration, Instant},
};
use tgbot::prelude::*;
use tokio::{spawn, sync::Mutex, time::delay_for};

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
async fn longpoll() {
    dotenv().ok();
    env_logger::init();
    let _m = mock("POST", "/bottoken/getUpdates")
        .match_body(Matcher::PartialJson(json!({
            "limit": 100,
            "timeout": 10,
            "allowed_updates": []
        })))
        .with_body(
            &serde_json::to_vec(&json!({
                "ok": true,
                "result": [{
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {
                            "id": 1,
                            "is_bot": false,
                            "first_name": "test"
                        },
                        "chat": {
                            "id": 1,
                            "type": "private",
                            "first_name": "test"
                        },
                        "text": "test"
                    }
                }]
            }))
            .unwrap(),
        )
        .create();
    let api = Api::new(Config::new("token").host(server_url())).unwrap();
    let updates = Arc::new(Mutex::new(Vec::new()));
    let handler = Handler {
        updates: updates.clone(),
    };
    let poll = LongPoll::new(api, handler);
    let handle = poll.get_handle();
    let wait_updates = updates.clone();
    spawn(async move {
        let now = Instant::now();
        while wait_updates.lock().await.is_empty() {
            if now.elapsed().as_secs() >= 2 {
                break;
            }
            delay_for(Duration::from_millis(100)).await;
        }
        handle.shutdown().await
    });
    poll.run().await;
    assert!(!updates.lock().await.is_empty())
}
