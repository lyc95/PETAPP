use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Cat {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub breed: String,
    pub birthdate: NaiveDate,
    pub photo_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCatRequest {
    pub name: String,
    pub breed: String,
    /// ISO 8601 date: "YYYY-MM-DD"
    pub birthdate: String,
    pub photo_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCatRequest {
    pub name: Option<String>,
    pub breed: Option<String>,
    /// ISO 8601 date: "YYYY-MM-DD"
    pub birthdate: Option<String>,
    pub photo_key: Option<String>,
}
