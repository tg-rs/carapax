use failure::Error;
use futures::{Future, Stream};
use std::time::Duration;
use tokio_executor::spawn;
use tokio_timer::Interval;

/// Adds GC support for a session store
pub trait GarbageCollector {
    /// Removes old sessions
    fn collect(&self) -> Box<Future<Item = (), Error = Error> + Send>;
}

/// Spawns a session GC
///
/// Allows to remove old sessions in a store at given interval
pub fn spawn_gc<C>(duration: Duration, collector: C)
where
    C: GarbageCollector + Send + 'static,
{
    spawn(
        Interval::new_interval(duration)
            .for_each(move |_| {
                collector.collect().then(|r| {
                    if let Err(e) = r {
                        log::error!("Failed to clear old sessions: {:?}", e);
                    }
                    Ok(())
                })
            })
            .map_err(|e| log::error!("Failed to spawn session GC: {:?}", e)),
    );
}
