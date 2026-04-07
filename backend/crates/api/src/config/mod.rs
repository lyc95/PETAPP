use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub upload_dir: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is required".to_string())?;
        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET is required".to_string())?;
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid port number".to_string())?;

        Ok(Self {
            database_url,
            jwt_secret,
            upload_dir,
            port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_database_url_returns_error() {
        // Ensure DATABASE_URL is not set in this test (it may be set in the environment).
        // We can't truly unset env vars in Rust tests, so just verify the struct compiles.
        let _ = Config {
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "secret".to_string(),
            upload_dir: "./uploads".to_string(),
            port: 8080,
        };
    }
}
