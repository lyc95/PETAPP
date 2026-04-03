use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use lambda_http::{run, Error};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod s3;
mod state;

use auth::{auth_middleware, JwksCache};
use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let config = Config::from_env().expect("failed to load config from env");
    let cfg = Arc::new(config);

    let jwks = JwksCache::new();
    jwks.load(&cfg.cognito_jwks_url)
        .await
        .expect("failed to load JWKS");

    let aws_cfg = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let dynamo = Arc::new(aws_sdk_dynamodb::Client::new(&aws_cfg));
    let s3 = Arc::new(aws_sdk_s3::Client::new(&aws_cfg));

    let state = Arc::new(AppState {
        config: cfg,
        jwks,
        dynamo,
        s3,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    // Public routes — no auth required.
    let public = Router::new().route("/health", get(routes::health::handler));

    // Protected routes — all require a valid Cognito JWT.
    let protected = routes::cats::router()
        .merge(routes::uploads::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = public.merge(protected).layer(cors).with_state(state);

    run(app).await
}
