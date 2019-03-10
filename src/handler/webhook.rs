use crate::{types::Update, UpdateHandler};
use futures::{future::ok, Future, Stream};
use hyper::{
    header::{HeaderValue, ALLOW},
    service::{MakeService, Service},
    Body, Error, Method, Request, Response, Server, StatusCode,
};
use std::{
    error::Error as StdError,
    fmt,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

/// Creates a webhook service
pub struct WebhookServiceFactory<H> {
    path: String,
    update_handler: Arc<Mutex<H>>,
}

impl<H> WebhookServiceFactory<H> {
    fn new<S: Into<String>>(path: S, update_handler: H) -> WebhookServiceFactory<H> {
        WebhookServiceFactory {
            path: path.into(),
            update_handler: Arc::new(Mutex::new(update_handler)),
        }
    }
}

/// An error when creating webhook service
#[derive(Debug)]
pub struct WebhookServiceFactoryError;

impl fmt::Display for WebhookServiceFactoryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Failed to create webhook service")
    }
}

impl StdError for WebhookServiceFactoryError {}

impl<Ctx, H> MakeService<Ctx> for WebhookServiceFactory<H>
where
    H: UpdateHandler + Send + Sync + 'static,
{
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = WebhookService<H>;
    type Future = Box<Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = WebhookServiceFactoryError;

    fn make_service(&mut self, _ctx: Ctx) -> Self::Future {
        Box::new(ok(WebhookService {
            path: self.path.clone(),
            update_handler: self.update_handler.clone(),
        }))
    }
}

/// Webhook service
pub struct WebhookService<H> {
    path: String,
    update_handler: Arc<Mutex<H>>,
}

impl<H> Service for WebhookService<H>
where
    H: UpdateHandler + Send + Sync + 'static,
{
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
            rep.headers_mut().insert(ALLOW, HeaderValue::from_static("POST"));
        }
        Box::new(ok(rep))
    }
}

pub(super) fn run_server<H>(addr: SocketAddr, path: String, handler: H)
where
    H: UpdateHandler + Send + Sync + 'static,
{
    let server = Server::bind(&addr)
        .serve(WebhookServiceFactory::new(path, handler))
        .map_err(|e| log::error!("Server error: {}", e));
    tokio::run(server)
}
