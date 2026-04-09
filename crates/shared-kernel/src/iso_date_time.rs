use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::DateTimeError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IsoDateTime(String);

impl IsoDateTime {
    pub fn now() -> Self {
        IsoDateTime(Utc::now().to_rfc3339())
    }

    pub fn new(value: String) -> Result<Self, DateTimeError> {
        chrono::DateTime::parse_from_rfc3339(&value)
            .map_err(|_| DateTimeError::ValidationError("invalid date format".to_string()))?;

        Ok(IsoDateTime(value))
    }

    pub fn utc(&self) -> Result<DateTime<Utc>, DateTimeError> {
        DateTime::parse_from_rfc3339(&self.0)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| DateTimeError::ValidationError("invalid date format".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

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

    #[test]
    fn convert_into_utc_datetime() {
        let datetime = IsoDateTime::new("2026-04-01T00:00:00Z".to_string()).unwrap();
        let utc_datetime: DateTime<Utc> = datetime.utc().unwrap();

        assert_eq!(
            utc_datetime,
            chrono::Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn reject_invalid_date_string_with_t_and_z() {
        let result = IsoDateTime::new("abcTZ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn reject_february_30() {
        let result = IsoDateTime::new("2026-02-30T00:00:00Z".to_string());
        assert!(result.is_err());
    }
}
