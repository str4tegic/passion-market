use crate::{errors::DomainError, ports::UserRepository};

pub trait EmailSpecification {
    fn is_satisfied_by(&self, email: &str) -> Result<(), DomainError>;
}

pub struct UniqueEmailSpecification<'repo, R: UserRepository> {
    repo: &'repo R,
}

impl<'repo, R: UserRepository> UniqueEmailSpecification<'repo, R> {
    pub fn new(repo: &'repo R) -> Self {
        Self { repo }
    }
}

impl<'repo, R: UserRepository> EmailSpecification for UniqueEmailSpecification<'repo, R> {
    fn is_satisfied_by(&self, email: &str) -> Result<(), DomainError> {
        if self.repo.find_by_email(email)?.is_some() {
            return Err(DomainError::Conflict("email already exists".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        password_hash::PasswordHash,
        user::{Role, User},
    };
    use shared_kernel::{
        ids::{IdentityId, new_id},
        iso_date_time::IsoDateTime,
    };

    struct FakeUserRepository {
        existing_emails: Vec<String>,
    }

    impl UserRepository for FakeUserRepository {
        fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
            if self.existing_emails.iter().any(|e| e == email) {
                let (user, _event) = User::register(
                    IdentityId(new_id()),
                    email.to_string(),
                    PasswordHash("hash".to_string()),
                    Role::Buyer,
                    IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap(),
                )?;
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }

        fn save(&self, _user: &User) -> Result<(), DomainError> {
            Ok(())
        }
    }

    #[test]
    fn email_existant_retourne_conflict() {
        let repo = FakeUserRepository {
            existing_emails: vec!["existant@test.com".to_string()],
        };
        let checker = UniqueEmailSpecification::new(&repo);
        let result = checker.is_satisfied_by("existant@test.com");
        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }

    #[test]
    fn email_libre_retourne_ok() {
        let repo = FakeUserRepository {
            existing_emails: vec![],
        };
        let checker = UniqueEmailSpecification::new(&repo);
        let result = checker.is_satisfied_by("nouveau@test.com");
        assert!(result.is_ok());
    }
}
