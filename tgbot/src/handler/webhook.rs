use crate::{types::Update, Never, UpdateHandler};
use futures::{
    future::{ok, Either},
    Future, Sink, Stream,
};
use hyper::{
    header::{HeaderValue, ALLOW},
    service::{MakeService, Service},
    Body, Error, Method, Request, Response, StatusCode,
};
use lazy_queue::sync::bounded::LazyQueue;
use tokio_executor::spawn;

#[doc(hidden)]
pub struct WebhookServiceFactory {
    path: String,
    queue: LazyQueue<Update>,
    processor: Option<Box<dyn Future<Item = (), Error = ()> + Send>>,
}

impl WebhookServiceFactory {
    #[doc(hidden)]
    pub fn new<S, H>(path: S, mut update_handler: H) -> WebhookServiceFactory
    where
        S: Into<String>,
        H: UpdateHandler + Send + Sync + 'static,
    {
        const QUEUE_SIZE: usize = 10;
        let (queue, processor) = LazyQueue::new(
            move |update| {
                update_handler.handle(update);
                Ok::<_, Never>(())
            },
            QUEUE_SIZE,
        );
        WebhookServiceFactory {
            path: path.into(),
            queue,
            processor: Some(Box::new(processor.map_err(|e| log::error!("Processing error: {}", e)))),
        }
    }
}

impl<Ctx> MakeService<Ctx> for WebhookServiceFactory {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = WebhookService;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = Never;

    fn make_service(&mut self, _ctx: Ctx) -> Self::Future {
        let path = self.path.clone();
        let queue = self.queue.clone();
        if let Some(fut) = self.processor.take() {
            spawn(fut);
        }
        Box::new(ok(WebhookService { path, queue }))
    }
}

#[doc(hidden)]
pub struct WebhookService {
    path: String,
    queue: LazyQueue<Update>,
}

fn put_on_a_queue(
    request: Request<Body>,
    queue: impl Sink<SinkItem = Update>,
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
            Err(err) => {
                log::error!(
                    "Failed to parse update \"{}\": {:?}",
                    err,
                    String::from_utf8_lossy(&body)
                );
                Either::B(ok(Response::new(Body::empty())))
            }
        })
}

impl Service for WebhookService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;

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
