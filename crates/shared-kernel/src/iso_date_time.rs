use serde::{Deserialize, Serialize};

use crate::errors::DateTimeError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IsoDateTime(String);

impl IsoDateTime {
    pub fn new(value: String) -> Result<Self, DateTimeError> {
        if !value.contains('T') || !value.ends_with('Z') {
            return Err(DateTimeError::ValidationError(
                "invalid date format".to_string(),
            ));
        }
        Ok(IsoDateTime(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_invalid_date_string() {
        let result = IsoDateTime::new("not-a-date".to_string());
        assert!(matches!(result, Err(DateTimeError::ValidationError(_))));
    }

    #[test]
    fn accept_valid_date_string() {
        let result = IsoDateTime::new("2026-04-01T00:00:00Z".to_string());
        assert!(result.is_ok());
    }
}
