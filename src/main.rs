use tracing_subscriber::EnvFilter;

use akeome2024 as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let input = lib::Input::read_from_file("tmp/input.json")?;
    tracing::info!(?input, "successfully read input");

    let due = input.due();
    let (sleep, rx) = lib::task::notify_on(due);
    let log = tokio::spawn(lib::task::log(rx.clone()));
    let send_messages: Vec<_> = input
        .traq_messages
        .iter()
        .map(|m| {
            let fut = lib::task::send_traq_message(rx.clone(), &input.traq_pat, m);
            tokio::spawn(fut)
        })
        .collect();
    let () = sleep.await??;
    let () = log.await??;
    for send_message in send_messages {
        send_message.await??;
    }
    Ok(())
}
