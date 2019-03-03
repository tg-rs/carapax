use crate::dispatcher::Dispatcher;
use crate::types::Update;
use futures::{future::ok, Future, Stream};
use hyper::{
    header::{HeaderValue, ALLOW},
    service::{MakeService, Service},
    Body, Error, Method, Request, Response, Server, StatusCode,
};
use std::error::Error as StdError;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

struct WebhookServiceFactory {
    path: String,
    update_handler: Arc<Mutex<Box<UpdateHandler + Send + Sync>>>,
}

impl WebhookServiceFactory {
    fn new<S: Into<String>>(
        path: S,
        update_handler: Box<UpdateHandler + Send + Sync>,
    ) -> WebhookServiceFactory {
        WebhookServiceFactory {
            path: path.into(),
            update_handler: Arc::new(Mutex::new(update_handler)),
        }
    }
}

#[derive(Debug)]
struct WebhookServiceFactoryError;

impl fmt::Display for WebhookServiceFactoryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Failed to create webhook service")
    }
}

impl StdError for WebhookServiceFactoryError {}

impl<Ctx> MakeService<Ctx> for WebhookServiceFactory {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = WebhookService;
    type Future = Box<Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = WebhookServiceFactoryError;

    fn make_service(&mut self, _ctx: Ctx) -> Self::Future {
        Box::new(ok(WebhookService {
            path: self.path.clone(),
            update_handler: self.update_handler.clone(),
        }))
    }
}

struct WebhookService {
    path: String,
    update_handler: Arc<Mutex<Box<UpdateHandler + Send + Sync>>>,
}

impl Service for WebhookService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Box<Future<Item = Response<Body>, Error = Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let mut rep = Response::new(Body::empty());
        if let Method::POST = *req.method() {
            if req.uri().path() == self.path {
                let update_handler = self.update_handler.clone();
                return Box::new(req.into_body().concat2().map(move |body| {
                    match serde_json::from_slice::<Update>(&body) {
                        Ok(update) => {
                            update_handler.lock().unwrap().handle(update);
                        }
                        Err(err) => {
                            *rep.status_mut() = StatusCode::BAD_REQUEST;
                            *rep.body_mut() = err.to_string().into();
                        }
                    }
                    rep
                }));
            } else {
                *rep.status_mut() = StatusCode::NOT_FOUND;
            }
        } else {
            *rep.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            rep.headers_mut()
                .insert(ALLOW, HeaderValue::from_static("POST"));
        }
        Box::new(ok(rep))
    }
}

/// A webhook update handler
pub trait UpdateHandler {
    /// Handles an update
    fn handle(&mut self, update: Update);
}

/// Handles update from webhook using dispatcher
pub struct WebhookDispatcher {
    dispatcher: Dispatcher,
}

impl WebhookDispatcher {
    /// Creates a handler
    pub fn new(dispatcher: Dispatcher) -> WebhookDispatcher {
        WebhookDispatcher { dispatcher }
    }
}

impl UpdateHandler for WebhookDispatcher {
    fn handle(&mut self, update: Update) {
        tokio::spawn(self.dispatcher.dispatch(&update).then(|r| {
            if let Err(e) = r {
                log::error!("Failed to dispatch update: {:?}", e)
            }
            Ok(())
        }));
    }
}

/// Starts a HTTP server for webhooks
///
/// # Arguments
///
/// - addr - Bind address
/// - path - URL path for webhook
/// - update_handler - An Update handler
pub fn run_server<A, S, H>(addr: A, path: S, update_handler: H)
where
    A: Into<SocketAddr>,
    S: Into<String>,
    H: UpdateHandler + Send + Sync + 'static,
{
    let server = Server::bind(&addr.into())
        .serve(WebhookServiceFactory::new(path, Box::new(update_handler)))
        .map_err(|e| log::error!("Server error: {}", e));
    tokio::run(server)
}
