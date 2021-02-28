use crate::{
    core::convert::{BoxedConvertFuture, ConvertHandler},
    FromUpdate, HandlerResult, ServiceUpdate,
};
use futures_util::future::BoxFuture;
use std::{
    error::Error,
    future::{ready, Future, Ready},
    marker::PhantomData,
};

/// Universality type of handler
pub type BoxedHandler = Box<dyn Handler<ServiceUpdate, BoxedConvertFuture> + Send>;

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

    /// Allows you to skip handler execution if false is returned
    ///
    /// ```rust,no_run
    /// # use carapax::types::Message;
    /// # use carapax::{Api, Dispatcher, Guard};
    /// use carapax::Handler;
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
    fn guard<F, R1, R2>(self, guard: F) -> Guard<Self, F, R1, R2>
    where
        Self: Sized,
    {
        Guard {
            handler: self,
            inner: guard,
            _r1: PhantomData,
            _r2: PhantomData,
        }
    }

    /// Wrap handler to box
    ///
    /// Conveniently for universality
    fn boxed(self) -> BoxedHandler
    where
        Self: Handler<T, R> + Send + Clone + 'static,
        T: FromUpdate + Send + 'static,
        T::Error: Error + Send,
        T::Future: Send,
        R: Future + Send + 'static,
        R::Output: Into<HandlerResult>,
    {
        ConvertHandler::boxed(self)
    }

    /// A dialogue that to be used in handler
    ///
    /// ```rust,no_run
    /// # use serde::{Serialize, Deserialize};
    /// use carapax::dialogue::{Dialogue, State, DialogueState};
    /// use carapax::session::backend::fs::FilesystemBackend;
    /// use carapax::{Api, Handler, Dispatcher};
    /// use carapax::methods::SendMessage;
    /// use carapax::types::Message;
    ///
    /// # type Result<T> = std::result::Result<T, carapax::ExecuteError>;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// enum MyState {
    ///     Start,
    ///     UserMessage,
    /// }
    ///
    /// impl Default for MyState {
    ///     fn default() -> Self {
    ///         Self::Start
    ///     }
    /// }
    ///
    /// impl State for MyState {
    ///     fn session_name() -> &'static str {
    ///         "super_session"
    ///     }
    /// }
    ///
    /// async fn my_handler(api: Api, message: Message, Dialogue { state, .. }: Dialogue<MyState, FilesystemBackend>) -> Result<DialogueState<MyState>> {
    ///     match state {
    ///         MyState::Start => {
    ///             api.execute(SendMessage::new(message.id, "Hi. Write something more!")).await?;
    ///             Ok(DialogueState::Next(MyState::UserMessage))
    ///         }
    ///         MyState::UserMessage => {
    ///             api.execute(SendMessage::new(message.id, format!("Your username is {}",
    ///                 message
    ///                     .get_user()
    ///                     .map(|user| user.username.as_deref())
    ///                     .flatten()
    ///                     .unwrap_or_default()))
    ///             ).await?;
    ///             Ok(DialogueState::Exit)
    ///         }
    ///     }
    /// }
    ///
    /// # let mut  dispatcher = Dispatcher::new(Api::new("123").unwrap());
    /// dispatcher.add_handler(my_handler.dialogue::<FilesystemBackend>());
    /// ```
    ///
    /// You can check what dialogue handler (like `my_handler` above) can return looking at
    /// [`DialogueResult` implementations](crate::dialogue::DialogueResult)
    #[cfg(feature = "dialogue")]
    fn dialogue<B>(self) -> crate::dialogue::DialogueHandler<Self, B, R>
    where
        Self: Sized,
    {
        crate::dialogue::DialogueHandler::from(self)
    }

    /// Wrapper for policy handler
    ///
    /// ```rust,no_run
    /// # use carapax::{Api, Dispatcher, Handler};
    /// use carapax::types::Message;
    ///
    /// async fn my_policy(message: Message) -> bool {
    ///     message.get_user().map(|user| user.id == 123).unwrap_or(false)
    /// }
    ///
    /// # let mut  dispatcher = Dispatcher::new(Api::new("123").unwrap());
    /// dispatcher.add_handler(my_policy.access());
    /// ```
    ///
    /// This method same as:
    /// ```rust,no_run
    /// # use carapax::{Dispatcher, Api};
    /// # async fn my_policy() -> bool { false }
    /// # trait InvertResult: Sized {
    /// #     fn invert_result(self) -> Self { self }
    /// # }
    /// # impl<T> InvertResult for T {}
    /// use carapax::{Handler, StopHandler};
    ///
    /// # let mut  dispatcher = Dispatcher::new(Api::new("123").unwrap());
    /// # dispatcher.add_handler(
    /// StopHandler.guard(my_policy).invert_result()
    /// # );
    /// ```
    /// where `invert_result()` is imaginary function that inverts [`HandlerResult`]:
    ///
    /// [`HandlerResult::Continue`] => [`HandlerResult::Stop`]
    ///
    /// [`HandlerResult::Stop`] => [`HandlerResult::Continue`]
    #[cfg(feature = "access")]
    fn access(self) -> crate::access::AccessHandler<Self, R>
    where
        Self: Sized,
    {
        crate::access::AccessHandler::from(self)
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
        impl<Func, $($T,)+ R> Handler<($($T,)+), R> for Func
        where
            Func: Fn($($T,)+) -> R,
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
impl_handler_for_fn!(A, B, C, D);
impl_handler_for_fn!(A, B, C, D, E);
impl_handler_for_fn!(A, B, C, D, E, F);
impl_handler_for_fn!(A, B, C, D, E, F, G);
impl_handler_for_fn!(A, B, C, D, E, F, G, H);
impl_handler_for_fn!(A, B, C, D, E, F, G, H, I); // 9 arguments

/// This utility trait is used to process what guard handler returned
///
/// See [`Handler::guard`]
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

/// See [`Handler::guard`]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ContinueHandler;

impl Handler<(), Ready<HandlerResult>> for ContinueHandler {
    fn call(&self, _param: ()) -> Ready<HandlerResult> {
        ready(HandlerResult::Continue)
    }
}

/// Handler that always returns [`HandlerResult::Stop`]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StopHandler;

impl Handler<(), Ready<HandlerResult>> for StopHandler {
    fn call(&self, _param: ()) -> Ready<HandlerResult> {
        ready(HandlerResult::Stop)
    }
}
