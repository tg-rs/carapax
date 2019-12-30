use crate::{session::Session, store::SessionStore};
use carapax::types::Update;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Registry for sessions
pub struct SessionFactory<S> {
    store: Arc<Mutex<S>>,
}

impl<S> SessionFactory<S>
where
    S: SessionStore,
{
    /// Creates a new registry with given store
    pub fn new(store: S) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }

    /// Returns a specific session store for given update
    pub fn from_update(&self, update: &Update) -> Session<S> {
        let namespace = namespace_from_update(update);
        Session::new(namespace, self.store.clone())
    }
}

fn namespace_from_update(update: &Update) -> String {
    let (chat_id, user_id) = match (update.get_chat_id(), update.get_user().map(|x| x.id)) {
        (Some(chat_id), Some(user_id)) => (chat_id, user_id),
        (Some(chat_id), None) => (chat_id, chat_id),
        (None, Some(user_id)) => (user_id, user_id),
        (None, None) => {
            // There should be always chat_id or user_id
            panic!("Can not get session namespace")
        }
    };
    format!("{}-{}", chat_id, user_id)
}
