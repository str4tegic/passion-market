use crate::{errors::DomainError, events::UserRegistered, password_hash::PasswordHash};
use serde::{Deserialize, Serialize};
use shared_kernel::ids::IdentityId;
use shared_kernel::iso_date_time::IsoDateTime;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Role {
    Maker,
    Buyer,
    Admin,
}

pub enum UserStatus {
    PendingValidation,
    Active,
}

pub struct User {
    pub id: IdentityId,
    pub email: String,
    pub password_hash: PasswordHash,
    pub role: Role,
    pub status: UserStatus,
    pub created_at: IsoDateTime,
}

impl User {
    pub fn register(
        id: IdentityId,
        email: String,
        password_hash: PasswordHash,
        role: Role,
        created_at: IsoDateTime,
    ) -> Result<(User, UserRegistered), DomainError> {
        let status = match role {
            Role::Maker | Role::Admin => UserStatus::PendingValidation,
            Role::Buyer => UserStatus::Active,
        };

        let user = User {
            id,
            email,
            password_hash,
            role,
            status,
            created_at,
        };

        let event = UserRegistered {
            id: user.id,
            email: user.email.clone(),
            role: user.role.clone(),
            occurred_at: user.created_at.clone(),
        };

        Ok((user, event))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared_kernel::ids::{IdentityId, new_id};

    fn date() -> IsoDateTime {
        IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap()
    }

    #[test]
    fn create_user_maker_with_status_is_pending_validation() {
        let (user, _event) = User::register(
            IdentityId(new_id()),
            "test@example.com".to_string(),
            PasswordHash("password123".to_string()),
            Role::Maker,
            date(),
        )
        .unwrap();

        assert!(matches!(user.status, UserStatus::PendingValidation));
    }

    #[test]
    fn create_user_buyer_with_status_is_active() {
        let (user, _event) = User::register(
            IdentityId(new_id()),
            "test@example.com".to_string(),
            PasswordHash("password123".to_string()),
            Role::Buyer,
            date(),
        )
        .unwrap();
        assert!(matches!(user.status, UserStatus::Active));
    }

    #[test]
    fn create_user_admin_with_status_is_pending_validation() {
        let (user, _event) = User::register(
            IdentityId(new_id()),
            "test@example.com".to_string(),
            PasswordHash("password123".to_string()),
            Role::Admin,
            date(),
        )
        .unwrap();
        assert!(matches!(user.status, UserStatus::PendingValidation));
    }

    #[test]
    fn create_user_has_created_at() {
        let (user, _event) = User::register(
            IdentityId(new_id()),
            "test@example.com".to_string(),
            PasswordHash("password123".to_string()),
            Role::Buyer,
            date(),
        )
        .unwrap();
        assert_eq!(user.created_at, date());
    }

    #[test]
    fn register_produit_un_event_user_registered() {
        let (user, event) = User::register(
            IdentityId(new_id()),
            "maker@test.com".to_string(),
            PasswordHash("motdepasse123".to_string()),
            Role::Maker,
            date(),
        )
        .unwrap();

        assert_eq!(event.email, user.email);
        assert_eq!(event.id, user.id);
    }

    #[test]
    #[ignore] // TODO: implement reject() method and uncomment this test
    fn a_pending_validation_user_can_be_rejected() {
        // let user = User::register(
        //     IdentityId(new_id()),
        //     "test@example.com".to_string(),
        //     PasswordHash("password123".to_string()),
        //     Role::Admin
        // );

        //user.reject();

        //assert!(matches!(user.status, UserStatus::Rejected));
    }
}
