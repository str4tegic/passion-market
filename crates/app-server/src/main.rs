mod config;
mod db;
mod health;

use anyhow::Result;
use axum::Router;

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

    // TODO Story 2+ : ajouter les routers BC ici
    let app = Router::new().merge(health::router());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port)).await?;
    tracing::info!("listening on 0.0.0.0:{}", cfg.port);

    axum::serve(listener, app).await?;

    Ok(())
}
