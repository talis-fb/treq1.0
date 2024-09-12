use std::future::Future;

use tokio::sync::oneshot;

pub fn parse_future_to_oneshot_receiver<T: Send + 'static>(
    future: impl Future<Output = T> + Send + 'static,
) -> oneshot::Receiver<T> {
    let (tx, rx) = oneshot::channel::<T>();
    tokio::spawn(async move {
        let _ = tx.send(future.await);
    });
    rx
}
