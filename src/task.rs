use std::future::Future;

use anyhow::Context;
use tokio::sync::watch;
use tracing::instrument;

#[instrument]
pub fn notify_on(
    timestamp: super::Timestamp,
) -> (
    tokio::task::JoinHandle<anyhow::Result<()>>,
    watch::Receiver<()>,
) {
    let (tx, rx) = watch::channel(());
    let fut = async move {
        let now = super::now();
        let duration = (timestamp - now)
            .to_std()
            .context("failed to get std::time::Duration")?;
        std::thread::sleep(duration);
        tx.send(()).context("failed to send")?;
        Ok(())
    };
    tracing::info!("start sleeping");
    let handle = tokio::spawn(fut);
    (handle, rx)
}

#[instrument(skip_all)]
pub fn log(
    mut rx: watch::Receiver<()>,
) -> impl Future<Output = anyhow::Result<()>> + Send + 'static {
    let _ = rx.borrow_and_update();
    async move {
        let () = rx.changed().await.context("failed to update")?;
        tracing::info!("あけましておめでとうございます🎉");
        Ok(())
    }
}
