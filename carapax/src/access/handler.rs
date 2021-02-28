use crate::{core::HandlerResult, Guard, GuardResult, Handler, HandlerExt, StopHandler};
use futures_util::future::BoxFuture;
use std::future::{Future, Ready};

/// See [`HandlerExt::access`]
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
}
