use async_trait::async_trait;
use identity_application::ports::UserRepository;
use identity_domain::{
    errors::DomainError,
    password_hash::PasswordHash,
    user::{Role, User, UserStatus},
};
use shared_kernel::{ids::IdentityId, iso_date_time::IsoDateTime};
use sqlx::PgPool;
use uuid::Uuid;

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT id, email, password_hash, role, status, created_at
               FROM identity.users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(r) => {
                let role = match r.role.as_str() {
                    "Maker" => Role::Maker,
                    "Buyer" => Role::Buyer,
                    "Admin" => Role::Admin,
                    other => {
                        return Err(DomainError::ValidationError(format!(
                            "unknown role in DB: {other}"
                        )));
                    }
                };
                let status = match r.status.as_str() {
                    "PendingValidation" => UserStatus::PendingValidation,
                    "Active" => UserStatus::Active,
                    other => {
                        return Err(DomainError::ValidationError(format!(
                            "unknown status in DB: {other}"
                        )));
                    }
                };
                Ok(Some(User::reconstitute(
                    IdentityId(r.id),
                    r.email,
                    PasswordHash(r.password_hash),
                    role,
                    status,
                    IsoDateTime::new(r.created_at.to_rfc3339())
                        .map_err(|e| DomainError::ValidationError(e.to_string()))?,
                )))
            }
        }
    }

    async fn save(&self, user: User) -> Result<(), DomainError> {
        let role = match user.role {
            Role::Maker => "Maker",
            Role::Buyer => "Buyer",
            Role::Admin => "Admin",
        };
        let status = match user.status {
            UserStatus::PendingValidation => "PendingValidation",
            UserStatus::Active => "Active",
        };

        sqlx::query!(
            r#"INSERT INTO identity.users (id, email, password_hash, role, status, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
            user.id.0 as Uuid,
            user.email,
            user.password_hash.0,
            role,
            status,
            user.created_at
                .utc()
                .map_err(|e| DomainError::ValidationError(e.to_string()))?,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::Conflict("email already exists".to_string());
            }
            DomainError::ValidationError(e.to_string())
        })?;

        Ok(())
    }
}
