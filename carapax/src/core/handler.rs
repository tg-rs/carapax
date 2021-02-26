use crate::HandlerResult;
use futures::future::BoxFuture;
use std::{
    future::{ready, Future, Ready},
    marker::PhantomData,
};

/// Base utility trait that implemented for [`Fn`]
///
/// It returns future (generic `R`) and takes `param` (generic `T`)
/// which allows you to use async functions with any arguments that implement [`FromUpdate`]
///
/// [`FromUpdate`]: crate::FromUpdate
pub trait Handler<T, R> {
    #[allow(missing_docs)]
    fn call(&self, param: T) -> R;

    /// The name of handler which is used for debug purposes
    ///
    /// Uses [`type_name`] internally
    ///
    /// [`type_name`]: std::any::type_name
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<F, R> Handler<(), R> for F
where
    F: Fn() -> R,
{
    fn call(&self, _: ()) -> R {
        (self)()
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

/// Extension for [`Handler`] with utility functions
pub trait HandlerExt<T, R>: Sized {
    /// Allows you to skip handler execution if false is returned
    ///
    /// ```rust,no_run
    /// # use carapax::types::Message;
    /// # use carapax::{Api, Dispatcher, Guard};
    /// use carapax::HandlerExt;
    ///
    /// async fn handler(api: Api) {
    ///     // some code...
    /// }
    ///
    /// // guard is handler too!
    /// async fn my_guard(message: Message) -> bool {
    ///     message.id == 123 // execution will be continued if message id is 123
    /// }
    ///
    /// let my_handler = handler.guard(my_guard);
    /// # Dispatcher::new(unreachable!()).add_handler(my_handler);
    /// // or you can use closure
    /// let my_handler = handler.guard(|message: Message| async move { message.id == 123 });
    /// # Dispatcher::new(unreachable!()).add_handler(my_handler);
    /// # // ^^^ these lines with dispatcher are used to avoid error:
    /// # // cannot infer type for type parameter `R1` declared on the associated function `guard`
    /// ```
    ///
    /// You can return not only boolean, see [`GuardResult`] implementations
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

/// This utility trait is used to process what guard handler returned
///
/// See [`HandlerExt::guard`]
pub trait GuardResult<H> {
    #[allow(missing_docs)]
    type Future: Future<Output = HandlerResult>;

    #[allow(missing_docs)]
    fn process_handler(self, handler: H) -> Self::Future;
}

impl<H> GuardResult<H> for bool
where
    H: Future + Send + 'static,
    H::Output: Into<HandlerResult>,
{
    type Future = BoxFuture<'static, HandlerResult>;

    fn process_handler(self, handler: H) -> Self::Future {
        Box::pin(async move {
            if self {
                handler.await.into()
            } else {
                HandlerResult::Continue
            }
        })
    }
}

impl<H, E> GuardResult<H> for Result<bool, E>
where
    H: Future + Send + 'static,
    H::Output: Into<HandlerResult>,
    E: std::error::Error + Send + 'static,
{
    type Future = BoxFuture<'static, HandlerResult>;

    fn process_handler(self, handler: H) -> Self::Future {
        Box::pin(async move {
            match self {
                Ok(true) => handler.await.into(),
                Ok(false) => HandlerResult::Continue,
                Err(err) => HandlerResult::error(err),
            }
        })
    }
}

/// See [`HandlerExt::guard`]
pub struct Guard<H, F, R1, R2> {
    handler: H,
    inner: F,
    _r1: PhantomData<R1>,
    _r2: PhantomData<R2>,
}

impl<H, F, R1, R2> Clone for Guard<H, F, R1, R2>
where
    H: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            inner: self.inner.clone(),
            _r1: PhantomData,
            _r2: PhantomData,
        }
    }
}

impl<H, F, T1, T2, R1, R2> Handler<(T1, T2), BoxFuture<'static, HandlerResult>> for Guard<H, F, R1, R2>
where
    H: Handler<T1, R1>,
    F: Handler<T2, R2>,
    R1: Future + Send + 'static,
    R1::Output: Into<HandlerResult>,
    R2: Future + Send + 'static,
    R2::Output: GuardResult<R1> + Send,
    <R2::Output as GuardResult<R1>>::Future: Send,
{
    fn call(&self, (t1, t2): (T1, T2)) -> BoxFuture<'static, HandlerResult> {
        // TODO: make handler calling depends on result of inner
        let handler = self.handler.call(t1);
        let inner = self.inner.call(t2);
        Box::pin(async move {
            let proceed = inner.await;
            proceed.process_handler(handler).await
        })
    }

    fn name(&self) -> &'static str {
        #[derive(Default)]
        struct Guard<H, F> {
            _h: PhantomData<H>,
            _f: PhantomData<F>,
        }

        impl<H, F> Handler<(), ()> for Guard<H, F> {
            fn call(&self, _param: ()) {
                unreachable!()
            }
        }

        Guard::<H, F> {
            _h: PhantomData,
            _f: PhantomData,
        }
        .name()
    }
}

/// Handler that always returns [`HandlerResult::Continue`]
pub struct ContinueHandler;

impl Handler<(), Ready<HandlerResult>> for ContinueHandler {
    fn call(&self, _param: ()) -> Ready<HandlerResult> {
        ready(HandlerResult::Continue)
    }
}

/// Handler that always returns [`HandlerResult::Stop`]
pub struct StopHandler;

impl Handler<(), Ready<HandlerResult>> for StopHandler {
    fn call(&self, _param: ()) -> Ready<HandlerResult> {
        ready(HandlerResult::Stop)
    }
}
