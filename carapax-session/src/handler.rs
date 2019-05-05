use crate::{
    session::{namespace_from_update, Session},
    store::SessionStore,
};
use carapax::prelude::*;
use std::sync::Arc;

/// A session handler for carapax
///
/// This handler sets Session to context,
/// so you can use it in your handlers:
///
/// ```
/// use carapax::prelude::*;
/// use carapax_session::{store::redis::RedisSessionStore, Session};
///
/// fn handler(context: &mut Context, message: &Message) -> HandlerFuture {
///     let session = context.get::<Session<RedisSessionStore>>();
///     // do something with session...
///     HandlerResult::Continue.into()
/// }
/// ```
pub struct SessionHandler<S> {
    store: Arc<S>,
}

impl<S> SessionHandler<S>
where
    S: SessionStore,
{
    /// Creates a new handler with given store
    pub fn new(store: S) -> Self {
        Self { store: Arc::new(store) }
    }
}

impl<S> Handler for SessionHandler<S>
where
    S: SessionStore + Send + Sync + 'static,
{
    type Input = Update;
    type Output = ();

    fn handle(&self, context: &mut Context, update: Self::Input) -> Self::Output {
        let namespace = namespace_from_update(&update);
        context.set(Session::new(namespace, self.store.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionKey;
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
        handler.handle(&mut context, update);
        assert!(context.get_opt::<Session<Store>>().is_some());
    }
}
