mod config;
mod db;
mod health;

use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use identity_application::use_cases::RegisterUserUseCase;
use identity_infra::{
    argon2_hasher::Argon2PasswordHasher, sqlx_user_repository::SqlxUserRepository,
};

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

    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
    let hasher = Arc::new(Argon2PasswordHasher);
    let register_uc = Arc::new(RegisterUserUseCase::new(hasher, user_repo));

    let app = Router::new()
        .merge(health::router())
        .merge(identity_api::identity_router(register_uc));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port)).await?;
    tracing::info!("listening on 0.0.0.0:{}", cfg.port);

    axum::serve(listener, app).await?;

    Ok(())
}
