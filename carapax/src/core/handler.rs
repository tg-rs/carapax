use crate::HandlerResult;
use futures::future::BoxFuture;
use std::future::Future;
use std::marker::PhantomData;

pub trait Handler<T, R> {
    fn call(&self, param: T) -> R;

    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

macro_rules! impl_handler_for_fn {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<F, $($T,)+ R> Handler<($($T,)+), R> for F
        where
            F: Fn($($T,)+) -> R,
        {
            fn call(&self, ($($T,)+): ($($T,)+)) -> R {
                (self)($($T,)+)
            }
        }
    };
}

impl_handler_for_fn!(A);
impl_handler_for_fn!(A, B);
impl_handler_for_fn!(A, B, C);

pub trait HandlerExt<T, R>: Sized {
    fn guard<F, R1, R2>(self, guard: F) -> Guard<Self, F, R1, R2> {
        Guard {
            handler: self,
            inner: guard,
            _r1: PhantomData,
            _r2: PhantomData,
        }
    }
}

impl<H, T, R> HandlerExt<T, R> for H where H: Handler<T, R> {}

pub struct Guard<H, F, R1, R2> {
    handler: H,
    inner: F,
    _r1: PhantomData<R1>,
    _r2: PhantomData<R2>,
}

impl<H, F, T1, T2, R1, R2> Handler<(T1, T2), BoxFuture<'static, HandlerResult>> for Guard<H, F, R1, R2>
where
    H: Handler<T1, R1>,
    F: Handler<T2, R2>,
    R1: Future + Send + 'static,
    R1::Output: Into<HandlerResult>,
    R2: Future<Output = bool> + Send + 'static,
{
    fn call(&self, (t1, t2): (T1, T2)) -> BoxFuture<'static, HandlerResult> {
        // TODO: make handler calling depends on result of inner
        let handler = self.handler.call(t1);
        let inner = self.inner.call(t2);
        Box::pin(async move {
            let proceed = inner.await;
            if proceed {
                handler.await.into()
            } else {
                HandlerResult::Continue
            }
        })
    }
}

// TODO: impl for Result
