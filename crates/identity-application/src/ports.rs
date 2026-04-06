use async_trait::async_trait;
use identity_domain::{errors::DomainError, events::UserRegistered, password_hash::PasswordHash, user::User};
use shared_kernel::ids::IdentityId;

use crate::use_cases::RegisterUserCommand;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: User) -> Result<(), DomainError>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<PasswordHash, DomainError>;
}

#[async_trait]
pub trait RegisterUserPort: Send + Sync {
    async fn execute(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(IdentityId, UserRegistered), DomainError>;
}