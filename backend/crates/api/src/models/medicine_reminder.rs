use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicineReminder {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    /// "MEDICATION" | "NAIL_CUT" | "EAR_WASH"
    pub reminder_type: String,
    pub label: String,
    /// ISO 8601 datetime, e.g. "2026-05-01T10:00:00Z"
    pub scheduled_date: String,
    pub is_recurring: bool,
    pub interval_days: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMedicineReminderRequest {
    pub reminder_type: String,
    pub label: String,
    pub scheduled_date: String,
    pub is_recurring: bool,
    pub interval_days: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMedicineReminderRequest {
    pub reminder_type: Option<String>,
    pub label: Option<String>,
    pub scheduled_date: Option<String>,
    pub is_recurring: Option<bool>,
    pub interval_days: Option<u32>,
    pub is_active: Option<bool>,
}
