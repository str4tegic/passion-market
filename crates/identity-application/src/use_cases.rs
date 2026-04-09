use std::sync::Arc;

use identity_domain::{
    errors::DomainError,
    events::UserRegistered,
    password_hash::PasswordHash,
    user::{Role, User},
};

use shared_kernel::{
    ids::{IdentityId, new_id},
    iso_date_time::IsoDateTime,
};

use crate::{
    ports::{PasswordHasher, RegisterUserPort, UserRepository},
    specifications::{UniqueEmailChecker, UniqueEmailSpecification},
};

pub struct RegisterUserUseCase {
    password_hasher: Arc<dyn PasswordHasher>,
    user_repository: Arc<dyn UserRepository>,
    email_checker: Arc<dyn UniqueEmailChecker>,
}

pub struct RegisterUserCommand {
    email: String,
    password: String,
    role: Role,
}

impl RegisterUserCommand {
    pub fn new(email: String, password: String, role: String) -> Result<Self, DomainError> {
        let role = Role::try_from(role)?;
        Ok(Self {
            email,
            password,
            role,
        })
    }
}

impl RegisterUserUseCase {
    pub fn new(
        password_hasher: Arc<dyn PasswordHasher>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        let email_checker = Arc::new(UniqueEmailSpecification::new(Arc::clone(&user_repository)));
        Self {
            password_hasher,
            user_repository,
            email_checker,
        }
    }
}

#[async_trait::async_trait]
impl RegisterUserPort for RegisterUserUseCase {
    async fn execute(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(IdentityId, UserRegistered), DomainError> {
        self.email_checker.is_satisfied_by(&command.email).await?;

        let password_hash = self.password_hasher.hash_password(&command.password)?;

        let (user, event) = User::register(
            IdentityId(new_id()),
            command.email,
            password_hash,
            command.role,
            IsoDateTime::now(),
        )?;

        let id = user.id.clone();
        self.user_repository.save(user).await?;

        Ok((id, event))
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use identity_domain::{
        password_hash::PasswordHash,
        user::{Role, User, UserStatus},
    };
    use shared_kernel::{
        ids::{IdentityId, new_id},
        iso_date_time::IsoDateTime,
    };

    use super::*;

    struct MockUserRepository {
        existing_email: Option<String>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
            if self.existing_email.as_deref() == Some(email) {
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

        async fn save(&self, _user: User) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct MockPasswordHasher;

    impl PasswordHasher for MockPasswordHasher {
        fn hash_password(&self, _password: &str) -> Result<PasswordHash, DomainError> {
            Ok(PasswordHash("hashed_password".to_string()))
        }
    }

    fn make_uc(existing_email: Option<&str>) -> RegisterUserUseCase {
        let repo = Arc::new(MockUserRepository {
            existing_email: existing_email.map(str::to_string),
        });
        RegisterUserUseCase {
            password_hasher: Arc::new(MockPasswordHasher),
            user_repository: Arc::clone(&repo) as Arc<dyn UserRepository>,
            email_checker: Arc::new(UniqueEmailSpecification::new(
                Arc::clone(&repo) as Arc<dyn UserRepository>
            )),
        }
    }

    fn cmd() -> RegisterUserCommand {
        RegisterUserCommand::new(
            "test@example.com".to_string(),
            "StrongPassword123".to_string(),
            "Buyer".to_string(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn execute_delegue_a_la_specification_pour_unicite_email() {
        // Le Conflict remonte via UniqueEmailSpecification → repo, pas via execute() directement.
        let result = make_uc(Some("test@example.com")).execute(cmd()).await;
        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }

    #[tokio::test]
    async fn inscription_reussie_retourne_identity_id() {
        let result = make_uc(None).execute(cmd()).await;
        assert!(matches!(result, Ok(_)));
    }

    #[tokio::test]
    async fn inscription_reussie_retourne_event_user_registered() {
        let result = make_uc(None).execute(cmd()).await;
        let (_, event) = result.unwrap();
        assert_eq!(event.email, "test@example.com");
    }
}
