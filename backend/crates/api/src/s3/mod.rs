use std::time::Duration;

use aws_sdk_s3::{presigning::PresigningConfig, Client};

pub async fn presign_put(
    s3: &Client,
    bucket: &str,
    key: &str,
    content_type: &str,
    ttl: Duration,
) -> anyhow::Result<String> {
    let config = PresigningConfig::expires_in(ttl)?;
    let presigned = s3
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type(content_type)
        .presigned(config)
        .await?;
    Ok(presigned.uri().to_string())
}

pub async fn presign_get(s3: &Client, bucket: &str, key: &str, ttl: Duration) -> anyhow::Result<String> {
    let config = PresigningConfig::expires_in(ttl)?;
    let presigned = s3
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(config)
        .await?;
    Ok(presigned.uri().to_string())
}
