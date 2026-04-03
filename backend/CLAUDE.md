# Rust Backend

## Language and Framework
- Rust edition 2021. Target: `aarch64-unknown-linux-musl` for Lambda.
- Web framework: axum 0.7. Runtime: lambda_http.
- Serialization: serde with `#[serde(rename_all = "camelCase")]` on all API-facing structs.

## Route Handlers
- Every handler receives `Extension<AuthContext>` from JWT middleware.
- Extract `owner_id` from `AuthContext` and use it in every DynamoDB query.
- Return `axum::Json<ApiResponse<T>>` for success, `AppError` for errors.
- Use `StatusCode::CREATED` (201) for POST, `StatusCode::NO_CONTENT` (204) for DELETE.

## DynamoDB Repos
- Each entity has its own repo file in `db/`.
- Read the table name from environment variable, never hardcode.
- Use `aws_sdk_dynamodb::Client` shared via axum `Extension`.
- Always filter by `owner_id` in queries — no exceptions.
- Use `put_item` for create, `update_item` for update, `delete_item` for delete.
- Use GSI queries (not `scan`) for listing records.

## Models
- Domain structs go in `models/` with one file per entity.
- Separate `CreateXxxRequest`, `UpdateXxxRequest`, and domain struct.
- Use `uuid::Uuid` for IDs, `chrono::DateTime<Utc>` for timestamps.

## Error Handling
- Define `AppError` enum in `errors/mod.rs` using `thiserror`.
- Implement `IntoResponse` for `AppError` to return the standard error envelope.
- Map AWS SDK errors to `AppError::Internal`.
- Map missing items to `AppError::NotFound`.
- Map bad input to `AppError::BadRequest` with a message.

## Auth
- JWT validation in `auth/middleware.rs` using `jsonwebtoken` crate.
- Fetch Cognito JWKS on startup, cache in memory.
- Extract `sub` claim as `owner_id`.

## Logging
- Use `tracing::info!`, `tracing::warn!`, `tracing::error!`.
- Never use `println!` or `eprintln!`.
- Initialize `tracing_subscriber` with env filter in `main.rs`.

## S3 Pre-signed URLs
- Generate upload URLs in `s3/mod.rs` using `aws_sdk_s3::presigning`.
- Set upload URL TTL to 15 minutes.
- Set download URL TTL to 15 minutes.
- Store only the S3 object key in DynamoDB, never the full URL.

## Testing
- Write unit tests for repo and model logic.
- Use `#[tokio::test]` for async tests.
- Run `cargo clippy -- -D warnings` and `cargo fmt --check` before finishing.
- Use `cargo lambda watch` for local dev (serves at http://localhost:9000).
- See `CatsApp.md` for integration testing and deployment instructions.
