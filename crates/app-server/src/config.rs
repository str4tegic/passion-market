use anyhow::Result;

/// Configuration globale de l'application.
/// Les champs seront utilisés dans les stories suivantes (2+).
#[allow(dead_code)]
#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
        if database_url.is_empty() {
            return Err(anyhow::anyhow!("DATABASE_URL must not be empty"));
        }

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev-secret-change-in-prod-minimum-32-chars".to_string());
        if jwt_secret.len() < 32 {
            return Err(anyhow::anyhow!("JWT_SECRET must be at least 32 characters"));
        }

        Ok(Self {
            database_url,
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://passion:passion@localhost:5672".to_string()),
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: std::env::var("S3_BUCKET").unwrap_or_else(|_| "passion-market".to_string()),
            s3_access_key: std::env::var("S3_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            s3_secret_key: std::env::var("S3_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            jwt_secret,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("PORT must be a valid u16"))?,
        })
    }
}
