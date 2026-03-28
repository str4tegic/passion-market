use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Génère un nouvel UUID v7
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

/// Newtype IDs par BC — utilisés dans les crates domaine pour un typage fort
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(pub Uuid);
