use crate::{handler::queue::Queue, types::Update, Never, UpdateHandler};
use futures::{
    future::{ok, Either},
    Future, Sink, Stream,
};
use hyper::{
    header::{HeaderValue, ALLOW},
    service::{MakeService, Service},
    Body, Error, Method, Request, Response, StatusCode,
};
use tokio_sync::mpsc;

/// Creates a webhook service
pub struct WebhookServiceFactory {
    path: String,
    queue: Queue,
}

impl WebhookServiceFactory {
    /// Creates a new factory
    pub fn new<S, H>(path: S, update_handler: H) -> WebhookServiceFactory
    where
        S: Into<String>,
        H: UpdateHandler + Send + Sync + 'static,
    {
        let queue = Queue::prepare(update_handler);
        WebhookServiceFactory {
            path: path.into(),
            queue,
        }
    }
}

impl<Ctx> MakeService<Ctx> for WebhookServiceFactory {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = WebhookService;
    type Future = Box<Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = Never;

    fn make_service(&mut self, _ctx: Ctx) -> Self::Future {
        let path = self.path.clone();
        let queue = self.queue.get_sender();
        self.queue.launch();
        Box::new(ok(WebhookService { path, queue }))
    }
}

/// Webhook service
pub struct WebhookService {
    path: String,
    queue: mpsc::Sender<Update>,
}

fn put_on_a_queue(
    request: Request<Body>,
    queue: mpsc::Sender<Update>,
) -> impl Future<Item = Response<Body>, Error = Error> {
    request
        .into_body()
        .concat2()
        .and_then(move |body| match serde_json::from_slice(&body) {
            Ok(update) => Either::A(queue.send(update).then(|res| {
                if res.is_err() {
                    log::warn!("The receiving end has been dropped");
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .expect("Can't construct an INTERNAL_SERVER_ERROR response"))
                } else {
                    Ok(Response::new(Body::empty()))
                }
            })),
            Err(err) => Either::B(ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err.to_string()))
                .expect("Can't construct a BAD_REQUEST response"))),
        })
}

impl Service for WebhookService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Box<Future<Item = Response<Body>, Error = Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        if let Method::POST = *req.method() {
            if req.uri().path() == self.path {
                Box::new(put_on_a_queue(req, self.queue.clone()))
            } else {
                Box::new(ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Can't construct a NOT_FOUND response")))
            }
        } else {
            Box::new(ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .header(ALLOW, HeaderValue::from_static("POST"))
                .body(Body::empty())
                .expect("Can't construct a METHOD_NOT_ALLOWED response")))
        }
    }
}
