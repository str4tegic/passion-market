mod config;
mod db;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string())
                .as_str(),
        )
        .init();

    let cfg = config::AppConfig::from_env()?;
    tracing::info!("passion-market app-server starting on port {}", cfg.port);

    let pool = db::create_pool(&cfg.database_url).await?;
    db::run_migrations(&pool).await?;
    tracing::info!("migrations applied successfully");

    // TODO Story 2+ : démarrer le serveur axum avec les routers BC
    tracing::info!("app-server ready — implémentation HTTP : Story 2+");

    Ok(())
}
