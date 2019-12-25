use crate::handler::UpdateHandler;
use bytes::buf::BufExt;
use futures_util::future::{ok, ready, FutureExt, Ready};
use http::Error as HttpError;
use hyper::{body, service::Service, Body, Error as HyperError, Method, Request, Response, Server, StatusCode};
use log::error;
use std::{
    convert::Infallible,
    error::Error as StdError,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

#[doc(hidden)]
pub struct WebhookServiceFactory<H> {
    path: String,
    handler: Arc<Mutex<H>>,
}

impl<H> WebhookServiceFactory<H> {
    #[doc(hidden)]
    pub fn new<P>(path: P, update_handler: H) -> Self
    where
        P: Into<String>,
    {
        WebhookServiceFactory {
            path: path.into(),
            handler: Arc::new(Mutex::new(update_handler)),
        }
    }
}

impl<H, T> Service<T> for WebhookServiceFactory<H> {
    type Response = WebhookService<H>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        let path = self.path.clone();
        ok(WebhookService {
            path,
            handler: self.handler.clone(),
        })
    }
}

#[doc(hidden)]
pub struct WebhookService<H> {
    path: String,
    handler: Arc<Mutex<H>>,
}

async fn handle_request<H>(
    handler: Arc<Mutex<H>>,
    path: String,
    request: Request<Body>,
) -> Result<Response<Body>, WebhookError>
where
    H: UpdateHandler,
{
    Ok(if let Method::POST = *request.method() {
        if request.uri().path() == path {
            let data = body::aggregate(request).await?;
            match serde_json::from_reader(data.reader()) {
                Ok(update) => {
                    let mut handler = handler.lock().await;
                    handler
                        .handle(update)
                        .await
                        .map_err(|err| WebhookError::Handler(format!("{:?}", err)))?;
                    Response::new(Body::empty())
                }
                Err(err) => Response::builder()
                    .header("Content-Type", "text/plain")
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!("Failed to parse update: {}\n", err)))?,
            }
        } else {
            Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty())?
        }
    } else {
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Allow", "POST")
            .body(Body::empty())?
    })
}

type ServiceFuture = Pin<Box<dyn Future<Output = Result<Response<Body>, WebhookError>> + Send>>;

impl<H> Service<Request<Body>> for WebhookService<H>
where
    H: UpdateHandler + Send + 'static,
{
    type Response = Response<Body>;
    type Error = WebhookError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(
            handle_request(self.handler.clone(), self.path.clone(), request).then(|result| match result {
                Ok(rep) => ok(rep),
                Err(err) => {
                    error!("Webhook error: {}", err);
                    ready(
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .map_err(WebhookError::from),
                    )
                }
            }),
        )
    }
}

#[doc(hidden)]
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum WebhookError {
    Hyper(HyperError),
    Http(HttpError),
    Handler(String),
}

impl StdError for WebhookError {
    fn cause(&self) -> Option<&dyn StdError> {
        use self::WebhookError::*;
        Some(match self {
            Hyper(err) => err,
            Http(err) => err,
            _ => return None,
        })
    }
}

/// Starts a server for webhook
///
/// # Arguments
///
/// * address - Bind address
/// * path - URL path for webhook
/// * handler - Updates handler
pub async fn run_server<A, P, H>(address: A, path: P, handler: H) -> Result<(), HyperError>
where
    A: Into<SocketAddr>,
    P: Into<String>,
    H: UpdateHandler + Send + 'static,
{
    let address = address.into();
    let path = path.into();
    let server = Server::bind(&address).serve(WebhookServiceFactory::new(path, handler));
    server.await
}
