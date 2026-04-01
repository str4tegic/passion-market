use serde::Serialize;
use shared_kernel::events::EventEnvelope;

use crate::{errors::DomainError, user::User};

pub trait UserRepository {
    fn save(&self, user: &User) -> Result<(), DomainError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
}

pub trait EventPublisher {
    fn publish<T: Serialize>(&self, event: EventEnvelope<T>) -> Result<(), DomainError>;
}
