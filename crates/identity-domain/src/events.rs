use crate::user::Role;
use serde::{Deserialize, Serialize};
use shared_kernel::ids::IdentityId;
use shared_kernel::iso_date_time::IsoDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistered {
    pub id: IdentityId,
    pub email: String,
    pub occurred_at: IsoDateTime,
    pub role: Role,
}

#[cfg(test)]
mod tests {

    use super::*;
    use shared_kernel::ids::new_id;

    #[test]
    fn create_event_user_registered() {
        let event = UserRegistered {
            id: IdentityId(new_id()),
            email: "test@example.com".to_string(),
            occurred_at: IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap(),
            role: Role::Buyer,
        };

        assert_eq!(event.email, "test@example.com");
        assert_eq!(
            event.occurred_at,
            IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap()
        );
        assert_eq!(event.role, Role::Buyer);
    }
}
