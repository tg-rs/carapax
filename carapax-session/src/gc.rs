use carapax::async_trait;
use std::{error::Error, time::Duration};
use tokio::time::interval;

/// Adds GC support for a session store
#[async_trait]
pub trait GarbageCollector {
    /// An error occurred when collecting garbage
    type Error: Error + Send + Sync;

    /// Removes old sessions
    async fn collect(&mut self) -> Result<(), Self::Error>;
}

/// Runs a session GC
///
/// Allows to remove old sessions in a store at given interval
///
/// # Arguments
///
/// * duration - A time interval between collect calls
/// * collector - Garbage collector
pub async fn run_gc<C>(duration: Duration, mut collector: C)
where
    C: GarbageCollector + Send + 'static,
{
    let mut interval = interval(duration);
    loop {
        interval.tick().await;
        if let Err(err) = collector.collect().await {
            log::error!("An error has occurred in session GC: {}", err)
        }
    }
}
