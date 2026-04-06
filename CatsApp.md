# Cat-Care App — Agent-Ready Plan

## Overview

Cross-platform mobile app to manage feeding, medication, weight tracking, and health records for cats.
Two devices: Android (owner) and iPhone (wife). AWS cloud platform. Rust backend.

---

## Prerequisites

Install these tools before starting:

| Tool | Purpose | Install |
|---|---|---|
| Node.js 20+ | React Native CLI, npm | https://nodejs.org |
| Rust (stable) | Backend compilation | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| cargo-lambda | Cross-compile Rust for Lambda | `cargo install cargo-lambda` |
| AWS CLI v2 | AWS account access | https://aws.amazon.com/cli/ |
| AWS SAM CLI | Build and deploy infra | https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html |
| Docker | Required by SAM for building Rust Lambda | https://www.docker.com |
| Android Studio | Android emulator + SDK | https://developer.android.com/studio |
| Xcode (macOS only) | iOS simulator | Mac App Store |
| Java 17+ | Required by React Native Android builds | https://adoptium.net |

Verify installation:
```bash
node --version          # v20+
rustc --version         # 1.75+
cargo lambda --version
aws --version           # 2.x
sam --version           # 1.x
docker --version
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Mobile App | React Native 0.74+ (TypeScript, strict mode) |
| UI Component Library | `react-native-paper` (Material Design 3) |
| Navigation | `@react-navigation/native` + `@react-navigation/stack` |
| Local Notifications | `@notifee/react-native` |
| Charts | `react-native-gifted-charts` |
| Image Picker | `react-native-image-picker` |
| HTTP Client | `axios` |
| Mobile Auth | `amazon-cognito-identity-js` |
| Backend Language | Rust (edition 2021) |
| Rust Web Framework | `axum 0.7` |
| Lambda Runtime | `lambda_http 0.13` |
| API Gateway | Amazon API Gateway HTTP API (v2) |
| Auth Provider | AWS Cognito User Pool |
| Database | Amazon DynamoDB |
| File Storage | Amazon S3 |
| Infra as Code | AWS SAM CLI |
| CI Target | `cargo-lambda` for cross-compilation to `aarch64-unknown-linux-musl` |

---

## Shared Access Model

Both users (you and your wife) must see the same cats and data. Options:

**Option A — Single shared Cognito account (recommended for v1)**
Both devices sign in to the same email/password. Same `ownerId` everywhere. Simplest path. No sharing logic needed.

**Option B — Household model (v2 if needed)**
Each user has their own Cognito account. Add a `householdId` field to all records. Both users belong to the same household. All queries filter by `householdId` instead of `ownerId`. Requires an invite/join flow.

Agent instruction: implement Option A first. All data models use `ownerId` which will be the same for both devices.

---

## Data Models

### TypeScript types (mobile)

```ts
// shared
type ReminderType = "MEDICATION" | "NAIL_CUT" | "EAR_WASH";
type HealthRecordType = "VACCINATION" | "VET_VISIT" | "NOTE";

interface Cat {
  id: string;
  ownerId: string;
  name: string;
  breed: string;
  birthdate: string;        // ISO 8601 date
  photoKey: string | null;  // S3 object key
  createdAt: string;        // ISO 8601 datetime
  updatedAt: string;
}

interface MealReminder {
  id: string;
  catId: string;
  ownerId: string;
  label: string;            // e.g. "Breakfast"
  scheduledTime: string;    // "HH:MM" 24h format
  daysOfWeek: number[];     // 0=Sun 1=Mon ... 6=Sat
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

interface MedicineReminder {
  id: string;
  catId: string;
  ownerId: string;
  reminderType: ReminderType;  // named reminderType (not type) to match backend camelCase serialization
  label: string;
  scheduledDate: string;    // ISO 8601 datetime
  isRecurring: boolean;
  intervalDays: number | null;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

interface WeightLog {
  id: string;
  catId: string;
  ownerId: string;
  weightKg: number;
  loggedAt: string;         // ISO 8601 datetime
  note: string | null;
  createdAt: string;
  updatedAt: string;
}

interface HealthRecord {
  id: string;
  catId: string;
  ownerId: string;
  type: HealthRecordType;
  title: string;
  description: string;
  recordedAt: string;       // ISO 8601 datetime
  attachmentKey: string | null;
  createdAt: string;
  updatedAt: string;
}
```

### Rust structs (backend)

```rust
use chrono::{NaiveDate, NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cat {
    pub id: Uuid,
    pub owner_id: String,
    pub name: String,
    pub breed: String,
    pub birthdate: NaiveDate,
    pub photo_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealReminder {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    pub label: String,
    pub scheduled_time: NaiveTime,
    pub days_of_week: Vec<u8>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReminderType {
    Medication,
    NailCut,
    EarWash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicineReminder {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    pub reminder_type: ReminderType,
    pub label: String,
    pub scheduled_date: DateTime<Utc>,
    pub is_recurring: bool,
    pub interval_days: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightLog {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    pub weight_kg: f64,
    pub logged_at: DateTime<Utc>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthRecordType {
    Vaccination,
    VetVisit,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthRecord {
    pub id: Uuid,
    pub cat_id: Uuid,
    pub owner_id: String,
    pub record_type: HealthRecordType,
    pub title: String,
    pub description: String,
    pub recorded_at: DateTime<Utc>,
    pub attachment_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## DynamoDB Table Design

Use separate tables. Each table uses `id` (String, UUID) as the partition key.

| Table | PK | GSI-1 | GSI-2 |
|---|---|---|---|
| `catcare-cats` | `id` | `ownerId-index` (PK: `ownerId`) | — |
| `catcare-meal-reminders` | `id` | `catId-index` (PK: `catId`) | `ownerId-index` (PK: `ownerId`) |
| `catcare-medicine-reminders` | `id` | `catId-index` (PK: `catId`) | `ownerId-index` (PK: `ownerId`) |
| `catcare-weight-logs` | `id` | `catId-loggedAt-index` (PK: `catId`, SK: `loggedAt`) | `ownerId-index` (PK: `ownerId`) |
| `catcare-health-records` | `id` | `catId-recordedAt-index` (PK: `catId`, SK: `recordedAt`) | `ownerId-index` (PK: `ownerId`) |

All tables use on-demand billing (PAY_PER_REQUEST). No provisioned capacity needed for this scale.

---

## API Design

Base path: `https://{api-id}.execute-api.{region}.amazonaws.com/`

### Standard response envelope

Success:
```json
{ "data": { ... } }
```

Success (list):
```json
{ "data": [ ... ], "count": 3 }
```

Error:
```json
{ "error": { "code": "NOT_FOUND", "message": "Cat not found" } }
```

### Standard HTTP status codes used
- 200 OK — read and update
- 201 Created — create
- 204 No Content — delete
- 400 Bad Request — validation error
- 401 Unauthorized — missing or invalid token
- 403 Forbidden — valid token but wrong owner
- 404 Not Found
- 500 Internal Server Error

### Health check
- `GET /health` — returns `{ "status": "ok" }`, no auth required

### Cats

`POST /cats`
```json
// Request
{ "name": "Mochi", "breed": "Scottish Fold", "birthdate": "2021-05-10" }
// Response 201
{ "data": { "id": "uuid", "ownerId": "...", "name": "Mochi", ... } }
```

`GET /cats` — returns all cats for the authenticated user
`GET /cats/{catId}` — returns one cat
`PATCH /cats/{catId}` — partial update, same shape as POST body (all fields optional)
`DELETE /cats/{catId}` — returns 204

### Meal reminders

`POST /cats/{catId}/meal-reminders`
```json
// Request
{ "label": "Breakfast", "scheduledTime": "07:30", "daysOfWeek": [1,2,3,4,5], "isActive": true }
// Response 201
{ "data": { "id": "uuid", "catId": "...", ... } }
```

`GET /cats/{catId}/meal-reminders` — list for one cat
`PATCH /meal-reminders/{id}` — partial update
`DELETE /meal-reminders/{id}` — 204

### Medicine reminders

`POST /cats/{catId}/medicine-reminders`
```json
// Request
{ "type": "MEDICATION", "label": "Dewormer", "scheduledDate": "2026-04-15T09:00:00Z", "isRecurring": true, "intervalDays": 90, "isActive": true }
```

`GET /cats/{catId}/medicine-reminders` — list for one cat
`PATCH /medicine-reminders/{id}` — partial update
`DELETE /medicine-reminders/{id}` — 204

### Weight logs

`POST /cats/{catId}/weight-logs`
```json
// Request
{ "weightKg": 4.2, "loggedAt": "2026-04-01T10:00:00Z", "note": "After breakfast" }
```

`GET /cats/{catId}/weight-logs` — list, sorted by `loggedAt` descending
`PATCH /weight-logs/{id}` — partial update
`DELETE /weight-logs/{id}` — 204

### Health records

`POST /cats/{catId}/health-records`
```json
// Request
{ "type": "VACCINATION", "title": "Rabies", "description": "Annual shot", "recordedAt": "2026-03-10T14:00:00Z" }
```

`GET /cats/{catId}/health-records` — list, sorted by `recordedAt` descending
`PATCH /health-records/{id}` — partial update
`DELETE /health-records/{id}` — 204

### File upload helpers

`POST /uploads/presign`
```json
// Request
{ "fileName": "mochi.jpg", "contentType": "image/jpeg" }
// Response
{ "data": { "uploadUrl": "https://s3...", "objectKey": "photos/uuid/mochi.jpg" } }
```

`GET /files/{encodedKey}/url` — returns `{ "data": { "downloadUrl": "https://s3..." } }` (pre-signed, 15 min TTL). The `encodedKey` must be Base64url-encoded by the client (not percent-encoded) because S3 keys contain slashes that conflict with path segments.

---

## Repository Structure

```
catcare/
+-- mobile/
|   +-- package.json
|   +-- tsconfig.json
|   +-- app.json
|   +-- index.js
|   +-- App.tsx
|   +-- src/
|   |   +-- navigation/
|   |   |   +-- RootNavigator.tsx
|   |   +-- screens/
|   |   |   +-- SignInScreen.tsx
|   |   |   +-- SignUpScreen.tsx
|   |   |   +-- HomeScreen.tsx         # dashboard + cat list + sign-out button
|   |   |   +-- AddCatScreen.tsx
|   |   |   +-- CatProfileScreen.tsx
|   |   |   +-- MealReminderScreen.tsx
|   |   |   +-- MedicineReminderScreen.tsx
|   |   |   +-- WeightLogScreen.tsx
|   |   |   +-- HealthRecordsScreen.tsx
|   |   +-- components/
|   |   |   +-- CatCard.tsx
|   |   |   +-- ReminderItem.tsx
|   |   |   +-- WeightChart.tsx
|   |   |   +-- HealthRecordItem.tsx
|   |   |   +-- DashboardSummary.tsx
|   |   +-- services/
|   |   |   +-- apiClient.ts           # axios instance, base URL, auth header injection; 401 interceptor refreshes token and retries once
|   |   |   +-- authService.ts         # Cognito sign-in, sign-up, token refresh
|   |   |   +-- notificationService.ts # Notifee scheduling helpers
|   |   |   +-- uploadService.ts       # presign + S3 upload helpers
|   |   +-- hooks/
|   |   |   +-- useCats.ts
|   |   |   +-- useReminders.ts
|   |   |   +-- useWeightLogs.ts
|   |   |   +-- useHealthRecords.ts
|   |   +-- types/
|   |   |   +-- index.ts               # all TypeScript interfaces from Data Models section
|   |   +-- theme/
|   |   |   +-- index.ts               # react-native-paper custom theme (colors, fonts)
|   |   +-- config/
|   |   |   +-- env.ts                 # API_BASE_URL, COGNITO_USER_POOL_ID, COGNITO_CLIENT_ID, S3_REGION
|
+-- backend/
|   +-- Cargo.toml                     # workspace root
|   +-- crates/
|   |   +-- api/
|   |   |   +-- Cargo.toml
|   |   |   +-- src/
|   |   |       +-- main.rs            # Lambda entry point, axum router setup
|   |   |       +-- lib.rs             # re-exports modules
|   |   |       +-- routes/
|   |   |       |   +-- mod.rs         # pub mod for each route module
|   |   |       |   +-- health.rs      # GET /health
|   |   |       |   +-- cats.rs
|   |   |       |   +-- meal_reminders.rs
|   |   |       |   +-- medicine_reminders.rs
|   |   |       |   +-- weight_logs.rs
|   |   |       |   +-- health_records.rs
|   |   |       |   +-- uploads.rs
|   |   |       +-- auth/
|   |   |       |   +-- mod.rs
|   |   |       |   +-- middleware.rs  # axum middleware: extract + validate Cognito JWT
|   |   |       |   +-- claims.rs     # JWT claims struct
|   |   |       +-- db/
|   |   |       |   +-- mod.rs
|   |   |       |   +-- client.rs     # shared DynamoDB client init
|   |   |       |   +-- cats_repo.rs
|   |   |       |   +-- meal_repo.rs
|   |   |       |   +-- medicine_repo.rs
|   |   |       |   +-- weight_repo.rs
|   |   |       |   +-- health_repo.rs
|   |   |       +-- models/
|   |   |       |   +-- mod.rs
|   |   |       |   +-- cat.rs
|   |   |       |   +-- meal_reminder.rs
|   |   |       |   +-- medicine_reminder.rs
|   |   |       |   +-- weight_log.rs
|   |   |       |   +-- health_record.rs
|   |   |       |   +-- api_response.rs  # envelope structs: ApiResponse<T>, ApiError
|   |   |       +-- errors/
|   |   |       |   +-- mod.rs         # AppError enum, impl IntoResponse
|   |   |       +-- config/
|   |   |       |   +-- mod.rs         # env var loading: TABLE_NAMES, S3_BUCKET, COGNITO_* 
|   |   |       +-- s3/
|   |   |           +-- mod.rs         # presigned URL generation helpers
|
+-- infra/
|   +-- template.yaml                  # AWS SAM template
|   +-- samconfig.toml
|   +-- env/
|       +-- dev.json                   # parameter overrides for dev
```

---

## Environment Variables (Lambda)

The Rust Lambda function reads these from the Lambda environment:

| Variable | Example | Purpose |
|---|---|---|
| `CATS_TABLE` | `catcare-cats` | DynamoDB table name |
| `MEAL_REMINDERS_TABLE` | `catcare-meal-reminders` | DynamoDB table name |
| `MEDICINE_REMINDERS_TABLE` | `catcare-medicine-reminders` | DynamoDB table name |
| `WEIGHT_LOGS_TABLE` | `catcare-weight-logs` | DynamoDB table name |
| `HEALTH_RECORDS_TABLE` | `catcare-health-records` | DynamoDB table name |
| `S3_BUCKET` | `catcare-uploads-dev` | S3 bucket for photos/attachments |
| `COGNITO_USER_POOL_ID` | `ap-southeast-1_xxxxxxxx` | For JWT validation |
| `COGNITO_JWKS_URL` | `https://cognito-idp.../.well-known/jwks.json` | JWKS endpoint |
| `RUST_LOG` | `info` | Tracing log level |

---

## SAM Template Skeleton

```yaml
AWSTemplateFormatVersion: "2010-09-09"
Transform: AWS::Serverless-2016-10-31
Description: Cat-Care Rust Backend

Globals:
  Function:
    Timeout: 10
    MemorySize: 256
    Runtime: provided.al2023
    Architectures:
      - arm64

Parameters:
  Stage:
    Type: String
    Default: dev

Resources:
  CatCareApi:
    Type: AWS::Serverless::HttpApi
    Properties:
      StageName: !Ref Stage
      CorsConfiguration:
        AllowOrigins:
          - "*"
        AllowHeaders:
          - Authorization
          - Content-Type
        AllowMethods:
          - GET
          - POST
          - PUT
          - PATCH
          - DELETE
          - OPTIONS

  CatCareFunction:
    Type: AWS::Serverless::Function
    Metadata:
      BuildMethod: rust-cargolambda
    Properties:
      CodeUri: ./backend
      Handler: bootstrap
      Environment:
        Variables:
          CATS_TABLE: !Ref CatsTable
          MEAL_REMINDERS_TABLE: !Ref MealRemindersTable
          MEDICINE_REMINDERS_TABLE: !Ref MedicineRemindersTable
          WEIGHT_LOGS_TABLE: !Ref WeightLogsTable
          HEALTH_RECORDS_TABLE: !Ref HealthRecordsTable
          S3_BUCKET: !Ref UploadsBucket
          COGNITO_USER_POOL_ID: !Ref CognitoUserPool
          COGNITO_JWKS_URL: !Sub "https://cognito-idp.${AWS::Region}.amazonaws.com/${CognitoUserPool}/.well-known/jwks.json"
          RUST_LOG: info
      Policies:
        - DynamoDBCrudPolicy:
            TableName: !Ref CatsTable
        - DynamoDBCrudPolicy:
            TableName: !Ref MealRemindersTable
        - DynamoDBCrudPolicy:
            TableName: !Ref MedicineRemindersTable
        - DynamoDBCrudPolicy:
            TableName: !Ref WeightLogsTable
        - DynamoDBCrudPolicy:
            TableName: !Ref HealthRecordsTable
        - S3CrudPolicy:
            BucketName: !Ref UploadsBucket
      Events:
        ApiCatchAll:
          Type: HttpApi
          Properties:
            ApiId: !Ref CatCareApi
            Path: /{proxy+}
            Method: ANY

  CognitoUserPool:
    Type: AWS::Cognito::UserPool
    Properties:
      UserPoolName: catcare-users
      AutoVerifiedAttributes:
        - email
      UsernameAttributes:
        - email
      Policies:
        PasswordPolicy:
          MinimumLength: 8

  CognitoUserPoolClient:
    Type: AWS::Cognito::UserPoolClient
    Properties:
      ClientName: catcare-app
      UserPoolId: !Ref CognitoUserPool
      ExplicitAuthFlows:
        - ALLOW_USER_SRP_AUTH
        - ALLOW_REFRESH_TOKEN_AUTH
      GenerateSecret: false

  CatsTable:
    Type: AWS::DynamoDB::Table
    Properties:
      TableName: !Sub "catcare-cats-${Stage}"
      BillingMode: PAY_PER_REQUEST
      AttributeDefinitions:
        - { AttributeName: id, AttributeType: S }
        - { AttributeName: ownerId, AttributeType: S }
      KeySchema:
        - { AttributeName: id, KeyType: HASH }
      GlobalSecondaryIndexes:
        - IndexName: ownerId-index
          KeySchema:
            - { AttributeName: ownerId, KeyType: HASH }
          Projection: { ProjectionType: ALL }

  MealRemindersTable:
    Type: AWS::DynamoDB::Table
    Properties:
      TableName: !Sub "catcare-meal-reminders-${Stage}"
      BillingMode: PAY_PER_REQUEST
      AttributeDefinitions:
        - { AttributeName: id, AttributeType: S }
        - { AttributeName: catId, AttributeType: S }
        - { AttributeName: ownerId, AttributeType: S }
      KeySchema:
        - { AttributeName: id, KeyType: HASH }
      GlobalSecondaryIndexes:
        - IndexName: catId-index
          KeySchema:
            - { AttributeName: catId, KeyType: HASH }
          Projection: { ProjectionType: ALL }
        - IndexName: ownerId-index
          KeySchema:
            - { AttributeName: ownerId, KeyType: HASH }
          Projection: { ProjectionType: ALL }

  MedicineRemindersTable:
    Type: AWS::DynamoDB::Table
    Properties:
      TableName: !Sub "catcare-medicine-reminders-${Stage}"
      BillingMode: PAY_PER_REQUEST
      AttributeDefinitions:
        - { AttributeName: id, AttributeType: S }
        - { AttributeName: catId, AttributeType: S }
        - { AttributeName: ownerId, AttributeType: S }
      KeySchema:
        - { AttributeName: id, KeyType: HASH }
      GlobalSecondaryIndexes:
        - IndexName: catId-index
          KeySchema:
            - { AttributeName: catId, KeyType: HASH }
          Projection: { ProjectionType: ALL }
        - IndexName: ownerId-index
          KeySchema:
            - { AttributeName: ownerId, KeyType: HASH }
          Projection: { ProjectionType: ALL }

  WeightLogsTable:
    Type: AWS::DynamoDB::Table
    Properties:
      TableName: !Sub "catcare-weight-logs-${Stage}"
      BillingMode: PAY_PER_REQUEST
      AttributeDefinitions:
        - { AttributeName: id, AttributeType: S }
        - { AttributeName: catId, AttributeType: S }
        - { AttributeName: loggedAt, AttributeType: S }
        - { AttributeName: ownerId, AttributeType: S }
      KeySchema:
        - { AttributeName: id, KeyType: HASH }
      GlobalSecondaryIndexes:
        - IndexName: catId-loggedAt-index
          KeySchema:
            - { AttributeName: catId, KeyType: HASH }
            - { AttributeName: loggedAt, KeyType: RANGE }
          Projection: { ProjectionType: ALL }
        - IndexName: ownerId-index
          KeySchema:
            - { AttributeName: ownerId, KeyType: HASH }
          Projection: { ProjectionType: ALL }

  HealthRecordsTable:
    Type: AWS::DynamoDB::Table
    Properties:
      TableName: !Sub "catcare-health-records-${Stage}"
      BillingMode: PAY_PER_REQUEST
      AttributeDefinitions:
        - { AttributeName: id, AttributeType: S }
        - { AttributeName: catId, AttributeType: S }
        - { AttributeName: recordedAt, AttributeType: S }
        - { AttributeName: ownerId, AttributeType: S }
      KeySchema:
        - { AttributeName: id, KeyType: HASH }
      GlobalSecondaryIndexes:
        - IndexName: catId-recordedAt-index
          KeySchema:
            - { AttributeName: catId, KeyType: HASH }
            - { AttributeName: recordedAt, KeyType: RANGE }
          Projection: { ProjectionType: ALL }
        - IndexName: ownerId-index
          KeySchema:
            - { AttributeName: ownerId, KeyType: HASH }
          Projection: { ProjectionType: ALL }

  UploadsBucket:
    Type: AWS::S3::Bucket
    Properties:
      BucketName: !Sub catcare-uploads-${Stage}-${AWS::AccountId}
      PublicAccessBlockConfiguration:
        BlockPublicAcls: true
        BlockPublicPolicy: true
        IgnorePublicAcls: true
        RestrictPublicBuckets: true
      CorsConfiguration:
        CorsRules:
          - AllowedHeaders: ["*"]
            AllowedMethods: [GET, PUT]
            AllowedOrigins: ["*"]
            MaxAge: 3600

Outputs:
  ApiUrl:
    Value: !Sub "https://${CatCareApi}.execute-api.${AWS::Region}.amazonaws.com/${Stage}"
  UserPoolId:
    Value: !Ref CognitoUserPool
  UserPoolClientId:
    Value: !Ref CognitoUserPoolClient
  UploadsBucketName:
    Value: !Ref UploadsBucket
```

---

## CORS Configuration

API Gateway HTTP API is configured with CORS in the SAM template above. The Rust `axum` app should also add CORS via `tower_http::cors::CorsLayer` for local development.

---

## React Native Paper Usage

Wrap the app root in `<PaperProvider>` with a shared theme:

```tsx
// App.tsx
import { PaperProvider } from 'react-native-paper';
import { theme } from './src/theme';

export default function App() {
  return (
    <PaperProvider theme={theme}>
      <RootNavigator />
    </PaperProvider>
  );
}
```

```ts
// src/theme/index.ts
import { MD3LightTheme } from 'react-native-paper';

export const theme = {
  ...MD3LightTheme,
  colors: {
    ...MD3LightTheme.colors,
    primary: '#6750A4',    // override as needed
  },
};
```

### Component mapping

| UI need | Paper component |
|---|---|
| Cat list cards | `<Card>`, `<Card.Cover>`, `<Card.Title>` |
| Form inputs | `<TextInput mode="outlined">` |
| Primary buttons | `<Button mode="contained">` |
| Add cat / add entry | `<FAB icon="plus">` |
| Confirmation dialogs | `<Dialog>` + `<Dialog.Actions>` |
| Success / error feedback | `<Snackbar>` |
| Screen headers | React Navigation header (not Paper) |
| Reminder toggle | `<Switch>` |
| Record type selector | `<SegmentedButtons>` |
| Loading indicator | `<ActivityIndicator>` |

---

## Mobile App Sync Strategy

No AppSync or WebSocket in v1. Instead:
1. Fetch all data for current screen on mount.
2. Refetch after every successful create, update, or delete mutation.
3. Refetch when a screen regains focus (React Navigation `useFocusEffect`).
4. Optional: add pull-to-refresh on list screens.

This is sufficient for two devices and a small dataset.

---

## Local Development Workflow

### Backend — local testing without deploying to AWS

Use `cargo lambda watch` to run the Rust API locally. It simulates the Lambda runtime on your machine.

```bash
cd backend

# Set env vars for local DynamoDB (or real AWS tables)
export CATS_TABLE=catcare-cats
export MEAL_REMINDERS_TABLE=catcare-meal-reminders
export MEDICINE_REMINDERS_TABLE=catcare-medicine-reminders
export WEIGHT_LOGS_TABLE=catcare-weight-logs
export HEALTH_RECORDS_TABLE=catcare-health-records
export S3_BUCKET=catcare-uploads-dev-123456789012
export COGNITO_USER_POOL_ID=ap-southeast-1_xxxxxxxx
export COGNITO_JWKS_URL=https://cognito-idp.ap-southeast-1.amazonaws.com/ap-southeast-1_xxxxxxxx/.well-known/jwks.json
export RUST_LOG=debug

cargo lambda watch
# API runs at http://localhost:9000
```

### Backend — local DynamoDB (optional, for fully offline dev)

```bash
# Run DynamoDB Local in Docker
docker run -p 8000:8000 amazon/dynamodb-local

# Create tables locally using AWS CLI
aws dynamodb create-table --endpoint-url http://localhost:8000 \
  --table-name catcare-cats \
  --attribute-definitions AttributeName=id,AttributeType=S \
  --key-schema AttributeName=id,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST
```

When using DynamoDB Local, set `AWS_ENDPOINT_URL=http://localhost:8000` in the Lambda env.

### Mobile — pointing to local backend

In `mobile/src/config/env.ts`, switch the API base URL:
```ts
// For local dev (Android emulator uses 10.0.2.2 to reach host)
export const API_BASE_URL = Platform.OS === 'android'
  ? 'http://10.0.2.2:9000'
  : 'http://localhost:9000';
```

### Testing with curl

```bash
# Health check (no auth)
curl http://localhost:9000/health

# Create cat (with auth token)
curl -X POST http://localhost:9000/cats \
  -H "Authorization: Bearer <your-cognito-token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "Mochi", "breed": "Scottish Fold", "birthdate": "2021-05-10"}'

# List cats
curl http://localhost:9000/cats \
  -H "Authorization: Bearer <your-cognito-token>"
```

---

## Testing Strategy

### Backend unit tests

```bash
cd backend
cargo test
```

What to test:
- Model serialization/deserialization (serde round-trip)
- Request validation logic
- Error mapping (AppError to HTTP status codes)
- DynamoDB attribute conversion helpers

### Backend integration tests

Test against real AWS services (using a separate `dev` stage):

```bash
# Deploy to dev
cd infra && sam deploy

# Run integration tests against the deployed API
cd backend
API_URL=https://xxxxx.execute-api.ap-southeast-1.amazonaws.com/dev cargo test --features integration
```

Integration tests should:
- Create a cat, read it back, update it, delete it
- Verify 401 on missing token
- Verify 404 on wrong ID
- Verify ownerId filtering (can't access another user's data)

### Mobile type checking

```bash
cd mobile
npx tsc --noEmit
```

### Manual testing checklist

| Test | How to verify |
|---|---|
| Sign up | Create account, verify email, sign in |
| Create cat | Add cat with photo, see it in list |
| Edit cat | Change name, verify update |
| Delete cat | Remove cat, verify gone from list |
| Meal reminder | Create reminder, wait for notification |
| Medicine reminder | Create one-off and recurring, verify notifications |
| Weight log | Add entry, check chart renders correctly |
| Health record | Add vaccination record, verify in list |
| Cross-device | Create cat on Android, pull-to-refresh on iPhone, verify it appears |
| Photo upload | Upload photo, close app, reopen, verify photo loads |
| Offline tolerance | Turn off wifi, open app, verify cached data shows (or graceful error) |

---

## Deployment Guide

### First-time setup (one-time)

1. **Configure AWS CLI**
   ```bash
   aws configure
   # Enter: AWS Access Key ID, Secret, Region (e.g. ap-southeast-1), Output format (json)
   ```

2. **Build the Rust Lambda binary**
   ```bash
   cd backend
   cargo lambda build --release --arm64
   ```

3. **Deploy with SAM**
   ```bash
   cd infra
   sam build
   sam deploy --guided
   ```
   SAM will ask:
   - Stack name: `catcare-dev`
   - Region: `ap-southeast-1` (or your preferred region)
   - Confirm changes: `y`
   - Allow IAM role creation: `y`
   - Save to `samconfig.toml`: `y`

4. **Note the outputs**
   After deploy, SAM prints:
   - `ApiUrl` — your API base URL
   - `UserPoolId` — your Cognito User Pool ID
   - `UserPoolClientId` — your Cognito App Client ID
   - `UploadsBucketName` — your S3 bucket name

5. **Update mobile config**
   Put the outputs into `mobile/src/config/env.ts`.

6. **Create your Cognito user**
   ```bash
   aws cognito-idp sign-up \
     --client-id <UserPoolClientId> \
     --username your@email.com \
     --password YourPassword123!
   
   aws cognito-idp admin-confirm-sign-up \
     --user-pool-id <UserPoolId> \
     --username your@email.com
   ```

### Subsequent deploys

```bash
cd backend && cargo lambda build --release --arm64
cd ../infra && sam build && sam deploy
```

### Tear down (delete everything)

```bash
cd infra
sam delete --stack-name catcare-dev
```
This removes all AWS resources (Lambda, tables, bucket, Cognito). Data will be lost.

---

## Implementation Phases

### Phase 1 — Project bootstrap
| Task | Files |
|---|---|
| Init React Native with TypeScript | `mobile/` (generated) |
| Install and configure `react-native-paper`: wrap `App.tsx` in `<PaperProvider>` with a custom theme; link `react-native-vector-icons` | `App.tsx`, `mobile/src/theme.ts` |
| Create Rust workspace | `backend/Cargo.toml`, `backend/crates/api/Cargo.toml` |
| Scaffold axum router with health check | `main.rs`, `lib.rs`, `routes/mod.rs`, `routes/health.rs` |
| Write SAM template and deploy empty Lambda | `infra/template.yaml`, `infra/samconfig.toml` |
| Verify `GET /health` returns 200 via API Gateway | — |

### Phase 2 — Authentication
| Task | Files |
|---|---|
| Deploy Cognito via SAM | `infra/template.yaml` (already included) |
| Build `SignInScreen` and `SignUpScreen` | `mobile/src/screens/SignInScreen.tsx`, `SignUpScreen.tsx` |
| Build `authService.ts` with Cognito SDK (sign-in, sign-up, sign-out, token refresh) | `mobile/src/services/authService.ts` |
| Build `apiClient.ts` that injects `Authorization` header; add 401 response interceptor to refresh token and retry once | `mobile/src/services/apiClient.ts` |
| Add sign-out button to `HomeScreen` | `screens/HomeScreen.tsx` |
| Build Rust JWT auth middleware | `auth/mod.rs`, `auth/middleware.rs`, `auth/claims.rs` |
| Protect all routes except `/health` with middleware | `main.rs` router setup |

### Phase 3 — Cats CRUD
| Task | Files |
|---|---|
| Rust: cats DynamoDB repo | `db/client.rs`, `db/cats_repo.rs` |
| Rust: cat model and request/response structs | `models/cat.rs`, `models/api_response.rs` |
| Rust: cats route handlers | `routes/cats.rs` |
| Rust: error handling | `errors/mod.rs` |
| Rust: config module (env vars) | `config/mod.rs` |
| Mobile: `AddCatScreen`, `CatProfileScreen`, `CatCard` | mobile screens + components |
| Mobile: `useCats` hook | `hooks/useCats.ts` |
| S3 photo upload: Rust presign endpoint | `routes/uploads.rs`, `s3/mod.rs` |
| S3 photo upload: mobile upload helper | `services/uploadService.ts` |

### Phase 4 — Reminders
| Task | Files |
|---|---|
| Rust: meal reminder repo + routes | `db/meal_repo.rs`, `routes/meal_reminders.rs`, `models/meal_reminder.rs` |
| Rust: medicine reminder repo + routes | `db/medicine_repo.rs`, `routes/medicine_reminders.rs`, `models/medicine_reminder.rs` |
| Mobile: `MealReminderScreen`, `MedicineReminderScreen` | mobile screens |
| Mobile: `ReminderItem` component | `components/ReminderItem.tsx` |
| Mobile: `useReminders` hook | `hooks/useReminders.ts` |
| Mobile: Notifee local notification scheduling | `services/notificationService.ts` |
| Mobile: Create Android notification channel on app launch (required by Notifee on Android) | `App.tsx` startup logic |
| Re-register notifications from backend data on app launch | `App.tsx` startup logic |

### Phase 5 — Weight and health records
| Task | Files |
|---|---|
| Rust: weight log repo + routes | `db/weight_repo.rs`, `routes/weight_logs.rs`, `models/weight_log.rs` |
| Rust: health record repo + routes | `db/health_repo.rs`, `routes/health_records.rs`, `models/health_record.rs` |
| Mobile: `WeightLogScreen`, `WeightChart` | mobile screen + component |
| Mobile: `HealthRecordsScreen`, `HealthRecordItem` | mobile screen + component |
| Mobile: `useWeightLogs`, `useHealthRecords` hooks | mobile hooks |
| Optional: health record attachment upload | reuse `uploadService.ts` flow |

### Phase 6 — Dashboard and sync
| Task | Files |
|---|---|
| Mobile: `HomeScreen` with `DashboardSummary` | `screens/HomeScreen.tsx`, `components/DashboardSummary.tsx` |
| Dashboard data: aggregate next meal, next med, last weight per cat | client-side logic in `HomeScreen` |
| Add `useFocusEffect` refetch to all list screens | each screen file |
| Verify both devices show consistent data | manual test |

### Phase 7 — Polish and optional enhancements
| Task | Files |
|---|---|
| Add pagination params to list endpoints (`limit`, `lastKey`) | Rust route handlers |
| Add input validation with meaningful error messages | Rust route handlers |
| Add pull-to-refresh on list screens | mobile screens |
| Optional: WebSocket or polling for tighter sync | future scope |

---

## Acceptance Criteria

| Feature | Criteria |
|---|---|
| Auth | User can sign up, sign in, sign out. Cognito token is sent on every API call. Expired tokens are auto-refreshed; on refresh failure the user is redirected to sign-in. |
| Rust API | `GET /health` returns 200. All CRUD endpoints return correct status codes and JSON envelope. |
| JWT Security | Requests without valid token get 401. Users can only access records matching their `ownerId`. |
| Cat Profile | Create, read, update, delete cats with photo upload to S3 via pre-signed URL. |
| Meal Reminder | Create, update, delete recurring meal reminders. Local notification fires on schedule. |
| Medicine Reminder | Create one-off or recurring medicine/nail/ear reminders. Local notification fires on schedule. |
| Weight Tracking | Log weight entries. Chart displays history sorted by date. |
| Health Records | Create, edit, delete records with optional attachment. |
| Cross-device Sync | Data updated on one device appears on the other after screen refresh. |
| Infrastructure | `sam build` and `sam deploy` provisions all resources successfully. |

---

## Key Dependencies

### Mobile (package.json)
```json
{
  "dependencies": {
    "react": "18.x",
    "react-native": "0.74.x",
    "react-native-paper": "^5.x",                         // Material Design 3 UI components
    "react-native-vector-icons": "^10.x",                 // required by react-native-paper for icons
    "@react-navigation/native": "^6.x",
    "@react-navigation/stack": "^6.x",
    "@notifee/react-native": "^7.x",
    "react-native-gifted-charts": "^1.x",
    "react-native-image-picker": "^7.x",
    "react-native-safe-area-context": "^4.x",
    "react-native-screens": "^3.x",
    "@react-native-async-storage/async-storage": "^1.x",  // required by amazon-cognito-identity-js for token persistence
    "amazon-cognito-identity-js": "^6.x",
    "axios": "^1.x"
  }
}
```

### Rust backend (Cargo.toml)
```toml
[workspace]
members = ["crates/api"]

[workspace.dependencies]
axum = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
aws-config = "1"
aws-sdk-dynamodb = "1"
aws-sdk-s3 = "1"
lambda_http = "0.13"
tower = "0.5"
tower-http = { version = "0.5", features = ["cors"] }
jsonwebtoken = "9"
```
