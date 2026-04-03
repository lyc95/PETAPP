# Cat-Care App

## Overview

Cross-platform mobile app with a Rust backend on AWS Lambda.
The full project plan (data models, API endpoints, DynamoDB design, implementation phases) is in `CatsApp.md`. Always reference it before implementing.

## Architecture

- **Mobile**: React Native (TypeScript, strict mode) in `mobile/`
- **Backend**: Rust (edition 2021, axum 0.7) on AWS Lambda in `backend/`
- **Infrastructure**: AWS SAM templates in `infra/`
- **Auth**: AWS Cognito. Mobile uses Cognito SDK directly. Backend validates JWT in middleware.
- **Database**: Amazon DynamoDB, separate tables per entity, on-demand billing.
- **Storage**: Amazon S3 for photos and attachments via pre-signed URLs.

## Conventions

- All dates and times use ISO 8601 format.
- All API responses use the standard JSON envelope: `{ "data": ... }` for success, `{ "error": { "code": "...", "message": "..." } }` for errors.
- Never hardcode AWS resource names. Always read from environment variables.
- Every DynamoDB query must filter by `ownerId` to enforce data isolation.
- Backend logging uses `tracing` crate, never `println!`.
- Mobile logging uses `console.warn` / `console.error`, never `alert()`.

## Build and Test

### Backend
```bash
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo lambda build --release --arm64
```

### Backend local dev
```bash
cd backend
cargo lambda watch
# API at http://localhost:9000
```

### Mobile
```bash
cd mobile
npm install
npx tsc --noEmit
npx react-native run-android
npx react-native run-ios
```

### Infrastructure
```bash
cd infra
sam build
sam deploy --guided
```

## How to Work on This Project

- Work in phases as defined in `CatsApp.md`. Complete one phase before starting the next.
- When building a backend endpoint: create model struct → DynamoDB repo → route handler → wire into router.
- When building a mobile screen: create screen component → data hook → wire into navigator.
- After creating or modifying Rust files, run `cargo clippy` and `cargo fmt`.
- After creating or modifying TypeScript files, run `npx tsc --noEmit`.
- Environment variables are listed in `.env.example`.
- See `CatsApp.md` for local dev workflow, testing strategy, and deployment guide.
