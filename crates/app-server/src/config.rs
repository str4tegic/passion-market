use anyhow::Result;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://passion:passion@localhost:5672".to_string()),
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: std::env::var("S3_BUCKET")
                .unwrap_or_else(|_| "passion-market".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-prod-minimum-32-chars".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("PORT must be a valid u16"))?,
        })
    }
}
