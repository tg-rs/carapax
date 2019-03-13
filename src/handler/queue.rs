use crate::{types::Update, UpdateHandler};
use futures::{Future, Stream};
use hyper::rt::spawn;
use tokio_sync::mpsc;

/// A lazy updates processing queue.
pub struct Queue {
    sender: mpsc::Sender<Update>,
    prepared_future: Option<Box<Future<Item = (), Error = ()> + Send>>,
}

impl Queue {
    /// Prepares a queue with a given update handler.
    ///
    /// To launch the processing run `Queue::launch` when you are inside a tokio's context.
    pub fn prepare<H>(mut update_handler: H) -> Self
    where
        H: UpdateHandler + Send + 'static,
    {
        const MAX_UPDATES_IN_QUEUE: usize = 10;
        let (sender, receiver) = mpsc::channel(MAX_UPDATES_IN_QUEUE);
        let processing = receiver
            .map_err(|_| log::error!("Channel receive error"))
            .for_each(move |update| {
                update_handler.handle(update);
                Ok(())
            });
        Queue {
            sender,
            prepared_future: Some(Box::new(processing)),
        }
    }

    /// Clones the underlying sender and gives it away.
    pub fn get_sender(&self) -> mpsc::Sender<Update> {
        self.sender.clone()
    }

    /// Launches processing of an updates queue.
    ///
    /// # Panics
    /// Should be launched from a tokio's context!
    pub fn launch(&mut self) {
        if let Some(fut) = self.prepared_future.take() {
            spawn(fut);
        }
    }
}
