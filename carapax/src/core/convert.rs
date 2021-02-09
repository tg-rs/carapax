use crate::core::{handler::Handler, result::HandlerResult};
use crate::{FromUpdate, ServiceUpdate};
use std::fmt::Display;
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

impl<H, T, R> Clone for ConvertHandler<H, T, R>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _t: PhantomData,
            _r: PhantomData,
        }
    }
}

impl<H, T, R> Handler<ServiceUpdate, BoxedConvertFuture> for ConvertHandler<H, T, R>
where
    H: Handler<T, R> + 'static + Send,
    T: FromUpdate + Send,
    T::Error: Display + 'static,
    R: Future + Send + 'static,
    R::Output: Into<HandlerResult>,
{
    fn call(&self, service_update: ServiceUpdate) -> BoxedConvertFuture {
        // TODO: describe what Ret for

        enum Ret<F> {
            Fut(F),
            HandlerResult(HandlerResult),
        }

        let ret = match T::from_update(service_update) {
            Ok(Some(t)) => Ret::Fut(self.handler.call(t)),
            Ok(None) => Ret::HandlerResult(HandlerResult::Continue),
            Err(err) => Ret::HandlerResult(HandlerResult::error(err)),
        };

        Box::pin(async move {
            match ret {
                Ret::Fut(fut) => fut.await.into(),
                Ret::HandlerResult(res) => res,
            }
        })
    }

    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
