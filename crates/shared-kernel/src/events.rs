use chrono::Utc;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

/// Enveloppeur standard pour tous les domain events publiés sur RabbitMQ.
///
/// Format JSON :
/// ```json
/// {
///   "event_type": "catalog.product.published",
///   "aggregate_id": "019123-...",
///   "occurred_at": "2026-03-28T12:00:00Z",
///   "version": 1,
///   "data": { ... }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct EventEnvelope<T> {
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub occurred_at: String, // ISO 8601 UTC
    pub version: u32,
    pub data: T,
}

impl<T: Serialize + DeserializeOwned> EventEnvelope<T> {
    pub fn new(event_type: impl Into<String>, aggregate_id: Uuid, data: T) -> Self {
        Self {
            event_type: event_type.into(),
            aggregate_id,
            occurred_at: Utc::now().to_rfc3339(),
            version: 1,
            data,
        }
    }
}
