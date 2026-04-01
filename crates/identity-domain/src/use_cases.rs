use crate::{
    domain_services::EmailSpecification,
    errors::DomainError,
    password_hash::PasswordHash,
    ports::{EventPublisher, UserRepository},
    user::{Role, User},
};
use shared_kernel::{events::EventEnvelope, ids::IdentityId, iso_date_time::IsoDateTime};
use uuid::Uuid;

pub struct RegisterUserCommand {
    id: IdentityId,
    email: String,
    password: PasswordHash,
    role: Role,
    registration_date: IsoDateTime,
}

impl RegisterUserCommand {
    pub fn new(
        id: Uuid,
        email: String,
        password: String,
        role: String,
        registration_date: String,
    ) -> Result<Self, DomainError> {
        let role = match role.as_str() {
            "Maker" => Role::Maker,
            "Buyer" => Role::Buyer,
            "Admin" => Role::Admin,
            _ => {
                return Err(DomainError::ValidationError(format!(
                    "invalid role: {role}"
                )));
            }
        };

        Ok(Self {
            id: IdentityId(id),
            email,
            password: PasswordHash::new(password)?,
            role,
            registration_date: IsoDateTime::new(registration_date)?,
        })
    }
}

pub fn register_user(
    repo: &impl UserRepository,
    publisher: &impl EventPublisher,
    unique_email: &impl EmailSpecification,
    cmd: RegisterUserCommand,
) -> Result<User, DomainError> {
    // Validation
    unique_email.is_satisfied_by(&cmd.email)?;

    // Create user + event (l'agrégat génère son propre event)
    let (user, domain_event) = User::register(
        cmd.id,
        cmd.email,
        cmd.password,
        cmd.role,
        cmd.registration_date,
    )?;

    // Save to repository
    repo.save(&user)?;

    // Publish event
    let envelope = EventEnvelope::new("identity.user.registered", user.id.0, domain_event);
    publisher.publish(envelope)?;

    Ok(user)
}
