use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// "VACCINATION" | "VET_VISIT" | "NOTE"
pub type HealthRecordType = String;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct HealthRecord {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: Uuid,
    pub record_type: HealthRecordType,
    pub title: String,
    pub description: String,
    /// ISO 8601 datetime — when the event occurred
    pub recorded_at: DateTime<Utc>,
    pub attachment_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHealthRecordRequest {
    pub record_type: HealthRecordType,
    pub title: String,
    pub description: String,
    pub recorded_at: String, // ISO 8601 datetime
    pub attachment_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHealthRecordRequest {
    pub record_type: Option<HealthRecordType>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub recorded_at: Option<String>,
    pub attachment_key: Option<String>,
}
