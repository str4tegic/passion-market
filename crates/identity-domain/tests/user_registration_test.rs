use identity_domain::{
    domain_services::UniqueEmailSpecification,
    errors::DomainError,
    password_hash::PasswordHash,
    ports::{EventPublisher, UserRepository},
    use_cases::{RegisterUserCommand, register_user},
    user::{Role, User},
};
use shared_kernel::{
    events::EventEnvelope,
    ids::{IdentityId, new_id},
    iso_date_time::IsoDateTime,
};
use std::cell::RefCell;

// --- FakeUserRepository ---

struct FakeUserRepository {
    existing_emails: Vec<String>,
}

impl FakeUserRepository {
    fn empty() -> Self {
        Self {
            existing_emails: vec![],
        }
    }

    fn with_email(email: &str) -> Self {
        Self {
            existing_emails: vec![email.to_string()],
        }
    }
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

// --- FakeEventPublisher ---

struct FakeEventPublisher {
    published_count: RefCell<u32>,
}

impl FakeEventPublisher {
    fn new() -> Self {
        Self {
            published_count: RefCell::new(0),
        }
    }

    fn count(&self) -> u32 {
        *self.published_count.borrow()
    }
}

impl EventPublisher for FakeEventPublisher {
    fn publish<T: serde::Serialize>(&self, _event: EventEnvelope<T>) -> Result<(), DomainError> {
        *self.published_count.borrow_mut() += 1;
        Ok(())
    }
}

fn cmd(email: &str, password: &str, role: &str) -> RegisterUserCommand {
    RegisterUserCommand::new(
        new_id(),
        email.to_string(),
        password.to_string(),
        role.to_string(),
        "2026-04-01T00:00:00Z".to_string(),
    )
    .unwrap()
}

// --- Tests ---

#[test]
fn inscription_reussie_cree_user_et_publie_event() {
    let repo = FakeUserRepository::empty();
    let publisher = FakeEventPublisher::new();
    let unique_email = UniqueEmailSpecification::new(&repo);

    let user = register_user(
        &repo,
        &publisher,
        &unique_email,
        cmd("maker@test.com", "motdepasse123", "Maker"),
    )
    .unwrap();

    assert_eq!(user.email, "maker@test.com");
    assert_eq!(publisher.count(), 1);
}

#[test]
fn email_deja_pris_retourne_conflict() {
    let repo = FakeUserRepository::with_email("maker@test.com");
    let publisher = FakeEventPublisher::new();
    let unique_email = UniqueEmailSpecification::new(&repo);

    let result = register_user(
        &repo,
        &publisher,
        &unique_email,
        cmd("maker@test.com", "motdepasse123", "Maker"),
    );

    assert!(matches!(result, Err(DomainError::Conflict(_))));
}

#[test]
fn mot_de_passe_trop_court_retourne_validation_error() {
    let result = RegisterUserCommand::new(
        new_id(),
        "maker@test.com".to_string(),
        "abc".to_string(),
        "Maker".to_string(),
        "2026-04-01T00:00:00Z".to_string(),
    );

    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}
