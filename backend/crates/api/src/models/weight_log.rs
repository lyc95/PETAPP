use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightLog {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    pub weight_kg: f64,
    /// ISO 8601 datetime — when the weight was measured
    pub logged_at: DateTime<Utc>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWeightLogRequest {
    pub weight_kg: f64,
    pub logged_at: String, // ISO 8601 datetime
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWeightLogRequest {
    pub weight_kg: Option<f64>,
    pub logged_at: Option<String>,
    pub note: Option<String>,
}
