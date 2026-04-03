use std::sync::Arc;

use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;

use crate::{auth::JwksCache, config::Config};

// Fields are wired into route handlers progressively across phases.
#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub jwks: JwksCache,
    pub dynamo: Arc<DynamoClient>,
    pub s3: Arc<S3Client>,
}
