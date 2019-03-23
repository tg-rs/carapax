use crate::{namespace::SessionNamespace, store::SessionStore};
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Update aware session store
///
/// Each key passed to session will be prefixed with namespace obtained from an update
/// Format of the namespace is: `(user-id|chat-id)-(user-id|chat-id)`
#[derive(Clone)]
pub struct Session<S> {
    namespace: SessionNamespace,
    store: Arc<S>,
}

impl<S> Session<S>
where
    S: SessionStore,
{
    pub(crate) fn new(namespace: SessionNamespace, store: Arc<S>) -> Self {
        Self { namespace, store }
    }
}
impl<S> SessionStore for Session<S>
where
    S: SessionStore,
{
    fn get<O>(&self, key: &str) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static,
    {
        self.store.get(&self.namespace.format_key(key))
    }

    fn set<I>(&self, key: &str, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize,
    {
        self.store.set(&self.namespace.format_key(key), val)
    }

    fn expire(&self, key: &str, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.expire(&self.namespace.format_key(key), seconds)
    }

    fn del(&self, key: &str) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.del(&self.namespace.format_key(key))
    }
}
