use crate::{
    core::{handler::Handler, result::HandlerResult},
    FromUpdate, ServiceUpdate,
};
use std::{future::Future, marker::PhantomData, pin::Pin};

pub(super) type BoxedConvertFuture = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

#[derive(Debug)]
pub(super) struct ConvertHandler<H, T, R> {
    handler: H,
    _t: PhantomData<T>,
    _r: PhantomData<R>,
}

impl<H, T, R> ConvertHandler<H, T, R> {
    pub(super) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self {
            handler,
            _t: PhantomData,
            _r: PhantomData,
        })
    }
}

impl<H, T, R> Handler<ServiceUpdate, BoxedConvertFuture> for ConvertHandler<H, T, R>
where
    H: Handler<T, R> + 'static + Send + Clone,
    T: FromUpdate + Send,
    T::Error: std::error::Error + 'static,
    T::Future: Send,
    R: Future + Send + 'static,
    R::Output: Into<HandlerResult>,
{
    fn call(&self, service_update: ServiceUpdate) -> BoxedConvertFuture {
        let handler = self.handler.clone();
        Box::pin(async move {
            match T::from_update(service_update).await {
                Ok(Some(t)) => {
                    let fut = handler.call(t);
                    tokio::pin!(fut);
                    fut.await.into()
                }
                Ok(None) => HandlerResult::Continue,
                Err(err) => HandlerResult::error(err),
            }
        })
    }

    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
