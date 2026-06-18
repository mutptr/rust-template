#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                let directives = if cfg!(debug_assertions) {
                    concat!("info,", env!("CARGO_PKG_NAME"), "=debug")
                } else {
                    "info"
                };
                tracing_subscriber::EnvFilter::new(directives)
            }),
        )
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S".to_owned(),
        ))
        .init();

    Ok(())
}
