use std::{error::Error, fmt};

use tokio::sync::Mutex;

use crate::{
    core::context::{Context, Ref},
    types::Update,
};

use super::*;

#[derive(Clone)]
struct UpdateStore(Arc<Mutex<Vec<Update>>>);

impl UpdateStore {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }

    async fn push(&self, update: Update) {
        self.0.lock().await.push(update)
    }

    async fn count(&self) -> usize {
        self.0.lock().await.len()
    }
}

async fn handler_ok(store: Ref<UpdateStore>, update: Update) {
    store.push(update).await;
}

async fn handler_error(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
    store.push(update).await;
    Err(HandlerError::new(ErrorMock))
}

#[derive(Debug)]
struct ErrorMock;

impl Error for ErrorMock {}

impl fmt::Display for ErrorMock {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Test error")
    }
}

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

#[tokio::test]
async fn chain() {
    macro_rules! assert_handle {
        ($strategy:ident, $count:expr, $($handler:expr),*) => {{
            let mut context = Context::default();
            context.insert(UpdateStore::new());
            let context = Arc::new(context);
            let mut chain = Chain::$strategy();
            $(chain = chain.with($handler);)*
            let update = create_update();
            let input = HandlerInput {
                context: context.clone(),
                update
            };
            let result = chain.handle(input).await;
            let count = context.get::<UpdateStore>().unwrap().count().await;
            assert_eq!(count, $count);
            result
        }};
    }

    let result = assert_handle!(all, 2, handler_ok, handler_error, handler_ok);
    assert!(result.is_err());
    let result = assert_handle!(once, 1, handler_ok, handler_error, handler_ok);
    assert!(matches!(result, Ok(())));

    let result = assert_handle!(all, 1, handler_error, handler_ok);
    assert!(result.is_err());
    let result = assert_handle!(once, 1, handler_error, handler_ok);
    assert!(result.is_err());

    let result = assert_handle!(all, 2, handler_ok, handler_ok);
    assert!(matches!(result, Ok(())));
    let result = assert_handle!(once, 1, handler_ok, handler_ok);
    assert!(matches!(result, Ok(())));
}
