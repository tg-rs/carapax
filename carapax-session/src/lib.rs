#![warn(missing_docs)]
//! A session handler for carapax

use self::handler::SessionHandler;
use carapax::Handler;
use store::SessionStore;

mod gc;
mod handler;
mod session;

pub use self::gc::{spawn_gc, GarbageCollector};
pub use self::session::{Session, SessionKey, SessionLifetime};

/// Contains session store implementations
pub mod store;

/// Creates a new session handler
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
pub fn session_handler<S>(store: S) -> Handler
where
    S: SessionStore + Send + Sync + 'static,
{
    Handler::update(SessionHandler::new(store))
}
