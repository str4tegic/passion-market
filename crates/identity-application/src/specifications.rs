use std::sync::Arc;

use async_trait::async_trait;
use identity_domain::errors::DomainError;

use crate::ports::UserRepository;

#[async_trait]
pub trait UniqueEmailChecker: Send + Sync {
    async fn is_satisfied_by(&self, email: &str) -> Result<(), DomainError>;
}

pub struct UniqueEmailSpecification {
    repo: Arc<dyn UserRepository>,
}

impl UniqueEmailSpecification {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UniqueEmailChecker for UniqueEmailSpecification {
    async fn is_satisfied_by(&self, email: &str) -> Result<(), DomainError> {
        if self.repo.find_by_email(email).await?.is_some() {
            return Err(DomainError::Conflict("email already exists".to_string()));
        }
        Ok(())
    }
}

  #[cfg(test)]
  mod tests {
      use super::*;
      use identity_domain::{
          password_hash::PasswordHash,
          user::{Role, User, UserStatus},
      };
      use shared_kernel::{ids::{IdentityId, new_id}, iso_date_time::IsoDateTime};

      struct MockRepo { existing: Option<String> }

      #[async_trait::async_trait]
      impl UserRepository for MockRepo {
          async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
              if self.existing.as_deref() == Some(email) {
                  Ok(Some(User::reconstitute(
                      IdentityId(new_id()),
                      email.to_string(),
                      PasswordHash("hash".to_string()),
                      Role::Buyer,
                      UserStatus::Active,
                      IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap(),
                  )))
              } else {
                  Ok(None)
              }
          }
          async fn save(&self, _user: User) -> Result<(), DomainError> { Ok(()) }
      }

      #[tokio::test]
      async fn email_existant_retourne_conflict() {
          let repo = MockRepo { existing: Some("taken@test.com".to_string()) };
          let spec = UniqueEmailSpecification::new(Arc::new(repo));
          let result = spec.is_satisfied_by("taken@test.com").await;
          assert!(matches!(result, Err(DomainError::Conflict(_))));
      }

      #[tokio::test]
      async fn email_libre_retourne_ok() {
          let repo = MockRepo { existing: None };
          let spec = UniqueEmailSpecification::new(Arc::new(repo));
          let result = spec.is_satisfied_by("new@test.com").await;
          assert!(result.is_ok());
      }
  }