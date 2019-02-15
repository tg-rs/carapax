use crate::methods::Request;
use futures::Future;

mod error;
mod hyper;

pub(crate) use self::error::ExecutorError;
pub(crate) use self::hyper::{default_executor, proxy_executor};

pub(crate) trait Executor {
    fn execute(&self, req: Request) -> Box<Future<Item = Vec<u8>, Error = ExecutorError>>;
}
