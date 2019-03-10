use anymap::{
    any::{Any, IntoBox},
    Map,
};
use failure::Error;
use std::net::SocketAddr;
use tgbot::{
    dispatcher::{DispatcherBuilder, ErrorStrategy, Handler, Middleware},
    webhook, Api,
};

/// Context for handlers
pub struct Context {
    inner: Map<Any + Send + Sync>,
}

impl Context {
    fn new() -> Self {
        Self { inner: Map::new() }
    }

    /// Adds a value to context
    pub fn add<T: IntoBox<Any + Send + Sync>>(&mut self, value: T) {
        self.inner.insert(value);
    }

    /// Get a value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get<T: IntoBox<Any + Send + Sync>>(&self) -> &T {
        self.inner.get().expect("Value not found in context")
    }

    /// Get a value from context
    ///
    /// Returns a reference to the value stored in context for the type T, if it exists
    pub fn get_opt<T: IntoBox<Any + Send + Sync>>(&self) -> Option<&T> {
        self.inner.get()
    }
}

/// Defines how to get updates
pub struct RunMethod {
    kind: RunMethodKind,
}

impl RunMethod {
    /// Get updates using long polling
    pub fn poll() -> Self {
        Self {
            kind: RunMethodKind::Poll,
        }
    }

    /// Get updates via webhook
    ///
    /// # Arguments
    ///
    /// - addr - Bind address
    /// - path - URL path for webhook
    pub fn webhook<A, S>(addr: A, path: S) -> Self
    where
        A: Into<SocketAddr>,
        S: Into<String>,
    {
        Self {
            kind: RunMethodKind::Webhook(addr.into(), path.into()),
        }
    }
}

enum RunMethodKind {
    Poll,
    Webhook(SocketAddr, String),
}

/// A Telegram Bot App
pub struct App {
    token: String,
    dispatcher_builder: DispatcherBuilder<Context>,
    context: Context,
    proxy: Option<String>,
}

impl App {
    /// Creates a new app
    ///
    /// # Arguments
    ///
    /// - token - A telegram bot token
    pub fn new<S: Into<String>>(token: S) -> Self {
        App {
            token: token.into(),
            dispatcher_builder: DispatcherBuilder::new(),
            proxy: None,
            context: Context::new(),
        }
    }

    /// Add a value to context
    pub fn context<T: IntoBox<Any + Send + Sync>>(mut self, value: T) -> Self {
        self.context.add(value);
        self
    }

    /// Set proxy for client
    pub fn proxy<S: Into<String>>(mut self, proxy: S) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Set middleware error strategy
    ///
    /// See `ErrorStrategy` for more information
    pub fn middleware_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.dispatcher_builder = self.dispatcher_builder.middleware_error_strategy(strategy);
        self
    }

    /// Set handler error strategy
    ///
    /// See `ErrorStrategy` for more information
    pub fn handler_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.dispatcher_builder = self.dispatcher_builder.handler_error_strategy(strategy);
        self
    }

    /// Add middleware handler
    pub fn add_middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware<Context> + 'static + Send + Sync,
    {
        self.dispatcher_builder = self.dispatcher_builder.add_middleware(middleware);
        self
    }

    /// Add a regular handler
    pub fn add_handler(mut self, handler: Handler<Context>) -> Self {
        self.dispatcher_builder = self.dispatcher_builder.add_handler(handler);
        self
    }

    /// Run app
    pub fn run(mut self, method: RunMethod) -> Result<(), Error> {
        let api = if let Some(proxy) = self.proxy {
            Api::with_proxy(self.token, &proxy)?
        } else {
            Api::new(self.token)?
        };
        self.context.add(api.clone());
        let dispatcher = self.dispatcher_builder.build(self.context);
        match method.kind {
            RunMethodKind::Poll => {
                dispatcher.start_polling(api);
            }
            RunMethodKind::Webhook(addr, path) => {
                webhook::run_server(addr, path, dispatcher);
            }
        }
        Ok(())
    }
}
