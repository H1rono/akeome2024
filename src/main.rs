use tracing_subscriber::EnvFilter;

use akeome2024 as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let due = lib::akeome_at();
    tracing::info!(%due, "Hello, world!");
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        let duration_until_due = lib::duration_until(due);
        tracing::info!(%duration_until_due, "tick");
    }
}
