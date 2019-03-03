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
use std::sync::{Arc, Mutex};

struct WebhookServiceFactory {
    path: String,
    dispatcher: Arc<Mutex<Dispatcher>>,
}

impl WebhookServiceFactory {
    fn new<S: Into<String>>(path: S, dispatcher: Dispatcher) -> WebhookServiceFactory {
        WebhookServiceFactory {
            path: path.into(),
            dispatcher: Arc::new(Mutex::new(dispatcher)),
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
            dispatcher: self.dispatcher.clone(),
        }))
    }
}

struct WebhookService {
    path: String,
    dispatcher: Arc<Mutex<Dispatcher>>,
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
                let dispatcher = self.dispatcher.clone();
                return Box::new(req.into_body().concat2().map(move |body| {
                    match serde_json::from_slice::<Update>(&body) {
                        Ok(update) => {
                            tokio::spawn(dispatcher.lock().unwrap().dispatch(&update).then(|r| {
                                if let Err(e) = r {
                                    log::error!("Failed to dispatch update: {:?}", e)
                                }
                                Ok(())
                            }));
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

/// Starts a HTTP server for webhooks
///
/// # Arguments
///
/// - addr - Bind address
/// - path - URL path for webhook
/// - dispatcher - A dispatcher
pub fn run_server<A, S>(addr: A, path: S, dispatcher: Dispatcher)
where
    A: Into<SocketAddr>,
    S: Into<String>,
{
    let server = Server::bind(&addr.into())
        .serve(WebhookServiceFactory::new(path, dispatcher))
        .map_err(|e| log::error!("Server error: {}", e));
    tokio::run(server)
}
