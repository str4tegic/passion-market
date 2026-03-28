use serde::{Deserialize, Serialize};

/// Montant en centimes — jamais de float pour les montants financiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_cents: u64,
    pub currency: &'static str,
}

impl Money {
    pub fn eur(amount_cents: u64) -> Self {
        Self {
            amount_cents,
            currency: "EUR",
        }
    }

    pub fn zero() -> Self {
        Self::eur(0)
    }
}
