use std::env;

// Fields are used progressively across phases; suppress dead-code until all routes exist.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub cats_table: String,
    pub meal_reminders_table: String,
    pub medicine_reminders_table: String,
    pub weight_logs_table: String,
    pub health_records_table: String,
    pub s3_bucket: String,
    pub cognito_user_pool_id: String,
    pub cognito_jwks_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        fn var(key: &str) -> Result<String, String> {
            env::var(key).map_err(|_| format!("missing env var: {key}"))
        }

        Ok(Self {
            cats_table: var("CATS_TABLE")?,
            meal_reminders_table: var("MEAL_REMINDERS_TABLE")?,
            medicine_reminders_table: var("MEDICINE_REMINDERS_TABLE")?,
            weight_logs_table: var("WEIGHT_LOGS_TABLE")?,
            health_records_table: var("HEALTH_RECORDS_TABLE")?,
            s3_bucket: var("S3_BUCKET")?,
            cognito_user_pool_id: var("COGNITO_USER_POOL_ID")?,
            cognito_jwks_url: var("COGNITO_JWKS_URL")?,
        })
    }

    /// Derives the Cognito issuer URL from the user pool ID.
    /// Pool IDs have the format "{region}_{id}", e.g. "us-east-1_AbCdEfGhI".
    pub fn cognito_issuer(&self) -> String {
        let region = self
            .cognito_user_pool_id
            .split('_')
            .next()
            .unwrap_or("us-east-1");
        format!(
            "https://cognito-idp.{}.amazonaws.com/{}",
            region, self.cognito_user_pool_id
        )
    }
}
