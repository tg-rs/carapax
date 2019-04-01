#![warn(missing_docs)]
//! A session handler for carapax

use self::handler::SessionHandler;
use carapax::Handler;
use store::SessionStore;

mod handler;
mod session;

pub use self::session::{Session, SessionKey};

/// Contains session store implementations
pub mod store;

/// Creates a new session handler
///
/// This handler sets Session to context,
/// so you can use it in your handlers like
/// ```ignore
/// let session = context.get::<Session<Store>>();
/// ```
pub fn session_handler<S>(store: S) -> Handler
where
    S: SessionStore + Send + Sync + 'static,
{
    Handler::update(SessionHandler::new(store))
}
