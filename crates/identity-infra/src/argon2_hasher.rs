use argon2::{
    Argon2, PasswordHasher as _,
    password_hash::{SaltString, rand_core::OsRng},
};
use identity_application::ports::PasswordHasher;
use identity_domain::{errors::DomainError, password_hash::PasswordHash};

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, raw_password: &str) -> Result<PasswordHash, DomainError> {
        PasswordHash::validate_password_strength(raw_password)?;

        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|_| DomainError::ValidationError("Failed to hash password".into()))?
            .to_string();
        Ok(PasswordHash(hash))
    }
}
