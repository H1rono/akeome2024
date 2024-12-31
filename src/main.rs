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
    let task = tokio::spawn(lib::task::log(rx.clone()));
    let () = sleep.await??;
    let () = task.await??;
    Ok(())
}
