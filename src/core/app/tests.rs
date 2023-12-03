use std::{error::Error, fmt};

use tokio::sync::Mutex;

use crate::core::{chain::Chain, context::Ref};

use super::*;

fn create_update() -> Update {
    serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1111,
            "date": 0,
            "from": {"id": 1, "is_bot": false, "first_name": "test"},
            "chat": {"id": 1, "type": "private", "first_name": "test"},
            "text": "test message from private chat"
        }
    }))
    .unwrap()
}

#[derive(Clone)]
struct Counter {
    value: Arc<Mutex<u8>>,
}

#[derive(Debug)]
struct ExampleError;

impl fmt::Display for ExampleError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Example error")
    }
}

impl Error for ExampleError {}

async fn success_handler(counter: Ref<Counter>) {
    *counter.value.lock().await += 1;
}

async fn error_handler(counter: Ref<Counter>) -> Result<(), ExampleError> {
    *counter.value.lock().await += 1;
    Err(ExampleError)
}

#[tokio::test]
async fn handle() {
    let counter = Counter {
        value: Arc::new(Mutex::new(0)),
    };

    let mut context = Context::default();
    context.insert(counter.clone());

    let chain = Chain::all().with(success_handler).with(error_handler);

    let app = App::new(context, chain);

    let update = create_update();
    app.handle(update).await;

    assert_eq!(*counter.value.lock().await, 2);
}
