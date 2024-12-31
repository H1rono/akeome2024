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

#[instrument(skip_all)]
pub fn send_traq_message(
    mut rx: watch::Receiver<()>,
    pat: &str,
    message: &super::Message,
) -> impl Future<Output = anyhow::Result<()>> + Send + 'static {
    let _ = rx.borrow_and_update();
    let super::Message { channel, content } = message;
    let client = reqwest::Client::new();
    let url = format!("https://q.trap.jp/api/v3/channels/{channel}/messages");
    let req = client
        .post(url)
        .bearer_auth(pat)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "content": content.to_string(),
            "embed": true
        }));
    async move {
        let () = rx.changed().await.context("failed to receive")?;
        let res = req
            .send()
            .await
            .context("failed to send")?
            .error_for_status()
            .context("received error response")?;
        tracing::info!(status = %res.status(), "sent message");
        Ok(())
    }
}

// FIXME: 403で失敗した。トークンの設定が良くなかった？
#[instrument(skip_all)]
pub fn merge_pull_request(
    mut rx: watch::Receiver<()>,
    pat: &str,
    pull_request: &super::PullRequest,
) -> impl Future<Output = anyhow::Result<()>> + Send + 'static {
    let _ = rx.borrow_and_update();
    let super::PullRequest {
        owner,
        repository,
        number,
    } = pull_request;
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{owner}/{repository}/pulls/{number}/merge");
    let req = client
        .put(url)
        .bearer_auth(pat)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "merge_method": "merge",
            "commit_message": "あけましておめでとうございます :tada:"
        }));
    async move {
        let () = rx.changed().await.context("failed to receive")?;
        let res = req
            .send()
            .await
            .context("failed to send")?
            .error_for_status()
            .context("received error response")?;
        tracing::info!(status = %res.status(), "merged pr");
        Ok(())
    }
}
