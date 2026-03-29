use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    EUR,
    USD,
}

/// Montant en centimes — jamais de float pour les montants financiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_cents: u64,
    pub currency: Currency,
}

impl Money {
    pub fn eur(amount_cents: u64) -> Self {
        Self { amount_cents, currency: Currency::EUR }
    }

    pub fn zero() -> Self {
        Self::eur(0)
    }
}
