use crate::errors::DomainError;

pub struct PasswordHash(pub String);

impl PasswordHash {
    pub fn validate_password_strength(password: &str) -> Result<(), DomainError> {
        if password.len() < 8 {
            return Err(DomainError::ValidationError(
                "password must be at least 8 characters long".to_string(),
            ));
        }
        Ok(())
    }

    pub fn new(password: String) -> Result<Self, DomainError> {
        Self::validate_password_strength(&password)?;
        // In a real implementation, you would hash the password here
        Ok(PasswordHash(password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_too_short_throws_error() {
        let result = PasswordHash::new("pass".to_string());

        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::ValidationError(_))));
    }

    #[test]
    fn password_long_enough() {
        let result = PasswordHash::new("motdepasse123".to_string());

        assert!(result.is_ok());
    }
}
