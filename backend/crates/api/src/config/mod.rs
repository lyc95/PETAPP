use std::env;

// Fields are used progressively across phases; suppress dead-code until all routes exist.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub local_mode: bool,
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
        fn opt_var(key: &str) -> String {
            env::var(key).unwrap_or_default()
        }

        let local_mode = env::var("LOCAL_MODE").map(|v| v == "true").unwrap_or(false);

        Ok(Self {
            local_mode,
            cats_table: var("CATS_TABLE")?,
            meal_reminders_table: var("MEAL_REMINDERS_TABLE")?,
            medicine_reminders_table: var("MEDICINE_REMINDERS_TABLE")?,
            weight_logs_table: var("WEIGHT_LOGS_TABLE")?,
            health_records_table: var("HEALTH_RECORDS_TABLE")?,
            s3_bucket: opt_var("S3_BUCKET"),
            cognito_user_pool_id: opt_var("COGNITO_USER_POOL_ID"),
            cognito_jwks_url: opt_var("COGNITO_JWKS_URL"),
        })
    }

    /// Derives the Cognito issuer URL from the user pool ID.
    /// Pool IDs have the format "{region}_{id}", e.g. "us-east-1_AbCdEfGhI".
    pub fn cognito_issuer(&self) -> String {
        let region = self
            .cognito_user_pool_id
            .split('_')
            .next()
            .unwrap_or("ap-southeast-1");
        format!(
            "https://cognito-idp.{}.amazonaws.com/{}",
            region, self.cognito_user_pool_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(pool_id: &str) -> Config {
        Config {
            local_mode: false,
            cats_table: "t".to_string(),
            meal_reminders_table: "t".to_string(),
            medicine_reminders_table: "t".to_string(),
            weight_logs_table: "t".to_string(),
            health_records_table: "t".to_string(),
            s3_bucket: "b".to_string(),
            cognito_user_pool_id: pool_id.to_string(),
            cognito_jwks_url: "u".to_string(),
        }
    }

    #[test]
    fn cognito_issuer_us_east_1() {
        assert_eq!(
            config("us-east-1_AbCdEfGhI").cognito_issuer(),
            "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_AbCdEfGhI"
        );
    }

    #[test]
    fn cognito_issuer_ap_southeast_1() {
        assert_eq!(
            config("ap-southeast-1_XyZ12345").cognito_issuer(),
            "https://cognito-idp.ap-southeast-1.amazonaws.com/ap-southeast-1_XyZ12345"
        );
    }
}
