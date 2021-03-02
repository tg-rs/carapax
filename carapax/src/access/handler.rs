use crate::{core::HandlerResult, Guard, GuardResult, Handler, StopHandler};
use futures_util::future::BoxFuture;
use std::future::{Future, Ready};

/// See [`Handler::access`]
pub struct AccessHandler<H, R>(Guard<StopHandler, H, Ready<HandlerResult>, R>);

impl<H: Clone, R> Clone for AccessHandler<H, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<H, R> From<H> for AccessHandler<H, R> {
    fn from(handler: H) -> Self {
        Self(StopHandler.guard(handler))
    }
}

impl<H, T, R> Handler<T, BoxFuture<'static, HandlerResult>> for AccessHandler<H, R>
where
    H: Handler<T, R>,
    R: Future + Send + 'static,
    R::Output: GuardResult<Ready<HandlerResult>> + Send,
    <R::Output as GuardResult<Ready<HandlerResult>>>::Future: Send,
{
    fn call(&self, param: T) -> BoxFuture<'static, HandlerResult> {
        let fut = self.0.call(((), param));
        Box::pin(async move {
            let res = fut.await;
            match res {
                HandlerResult::Continue => HandlerResult::Stop,
                HandlerResult::Stop => HandlerResult::Continue,
                HandlerResult::Error(err) => HandlerResult::Error(err),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::{Message, MessageData, MessageKind, PrivateChat, Update, UpdateKind, User},
        Api, Data, Dispatcher, UpdateHandler,
    };
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    async fn policy_true() -> bool {
        true
    }

    async fn policy_false() -> bool {
        false
    }

    #[tokio::test]
    async fn handler() {
        match policy_true.access().call(()).await {
            HandlerResult::Continue => { /*ok*/ }
            result => panic!("Unexpected handler result: {:?}", result),
        }

        match policy_false.access().call(()).await {
            HandlerResult::Stop => { /*ok*/ }
            result => panic!("Unexpected handler result: {:?}", result),
        }
    }

    struct NotPassedData;

    async fn not_passed_data(_data: Data<NotPassedData>) -> bool {
        true
    }

    async fn execution_check(data: Data<AtomicBool>) {
        data.store(true, Ordering::Relaxed);
    }

    #[tokio::test]
    async fn handler_error() {
        let flag = Arc::new(AtomicBool::new(false));

        let api = Api::new("123").unwrap();
        let mut dispatcher = Dispatcher::new(api);
        dispatcher
            .add_handler(not_passed_data.access())
            .add_handler(execution_check)
            .data_arc(flag.clone());

        let update = Update {
            id: 0,
            kind: UpdateKind::Message(Message {
                id: 0,
                date: 0,
                kind: MessageKind::Private {
                    chat: PrivateChat {
                        id: 0,
                        first_name: "".to_string(),
                        last_name: None,
                        username: None,
                        photo: None,
                        bio: None,
                        pinned_message: None,
                    },
                    from: User {
                        id: 0,
                        is_bot: false,
                        first_name: "".to_string(),
                        last_name: None,
                        username: None,
                        language_code: None,
                    },
                },
                forward: None,
                reply_to: None,
                via_bot: None,
                edit_date: None,
                media_group_id: None,
                data: MessageData::Empty,
                reply_markup: None,
                sender_chat: None,
            }),
        };
        dispatcher.handle(update).await;

        let flag = flag.load(Ordering::Relaxed);
        assert!(!flag);
    }
}
