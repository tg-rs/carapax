use crate::{
    session::{namespace_from_update, Session},
    store::SessionStore,
};
use carapax::prelude::*;
use std::sync::Arc;

pub(crate) struct SessionHandler<S> {
    store: Arc<S>,
}

impl<S> SessionHandler<S>
where
    S: SessionStore,
{
    pub(crate) fn new(store: S) -> Self {
        Self { store: Arc::new(store) }
    }
}

impl<S> UpdateHandler for SessionHandler<S>
where
    S: SessionStore + Send + Sync + 'static,
{
    fn handle(&self, context: &mut Context, update: &Update) -> HandlerFuture {
        let namespace = namespace_from_update(update);
        context.set(Session::new(namespace, self.store.clone()));
        HandlerResult::Continue.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionKey;
    use carapax::{core::types::Update, Context};
    use failure::Error;
    use futures::Future;
    use serde::{de::DeserializeOwned, Serialize};

    struct Store;

    impl SessionStore for Store {
        fn get<O>(&self, _key: SessionKey) -> Box<Future<Item = Option<O>, Error = Error> + Send>
        where
            O: DeserializeOwned + Send + 'static,
        {
            unimplemented!()
        }

        fn set<I>(&self, _key: SessionKey, _val: &I) -> Box<Future<Item = (), Error = Error> + Send>
        where
            I: Serialize,
        {
            unimplemented!()
        }

        fn expire(&self, _key: SessionKey, _seconds: usize) -> Box<Future<Item = (), Error = Error> + Send> {
            unimplemented!()
        }

        fn del(&self, _key: SessionKey) -> Box<Future<Item = (), Error = Error> + Send> {
            unimplemented!()
        }
    }

    #[test]
    fn handler() {
        let mut context = Context::default();
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                    "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                    "text": "test middleware"
                }
            }
        ))
        .unwrap();
        let handler = SessionHandler::new(Store);
        let result = handler.handle(&mut context, &update).wait().unwrap();
        assert_eq!(result, HandlerResult::Continue);
        assert!(context.get_opt::<Session<Store>>().is_some());
    }
}
