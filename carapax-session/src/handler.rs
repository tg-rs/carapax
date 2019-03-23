use crate::{
    namespace::SessionNamespace,
    store::{Session, SessionStore},
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
        let namespace = SessionNamespace::from_update(update);
        context.set(Session::new(namespace, self.store.clone()));
        HandlerResult::Continue.into()
    }
}
