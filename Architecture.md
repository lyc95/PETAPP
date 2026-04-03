# Cat-Care App — Architecture

## 1. System Architecture Overview

How the two phones connect to AWS services.

```mermaid
graph TB
    subgraph Devices["Mobile Devices"]
        Android["Android"]
        iPhone["iPhone"]
    end

    subgraph AWS["AWS Cloud"]
        APIGW["API Gateway\n(HTTP API v2)"]
        Cognito["Cognito\n(User Pool)"]

        subgraph Lambda["Lambda (Rust)"]
            Axum["axum Router"]
            JWT["JWT Middleware"]
            Handlers["Route Handlers"]
        end

        subgraph Storage["Data Layer"]
            DDB["DynamoDB\n(5 tables)"]
            S3["S3 Bucket\n(Photos and Files)"]
        end

        CW["CloudWatch\n(Logs)"]
    end

    Android -- "1. Sign in" --> Cognito
    iPhone -- "1. Sign in" --> Cognito
    Cognito -- "JWT Token" --> Android
    Cognito -- "JWT Token" --> iPhone

    Android -- "2. API calls\n(Bearer token)" --> APIGW
    iPhone -- "2. API calls\n(Bearer token)" --> APIGW

    APIGW --> JWT
    JWT --> Handlers
    Handlers -- "Read/Write" --> DDB
    Handlers -- "Pre-signed URLs" --> S3

    Android -- "3. Upload photos\n(pre-signed URL)" --> S3
    iPhone -- "3. Upload photos\n(pre-signed URL)" --> S3

    Lambda --> CW
```

### Flow summary

| Step | What happens |
|---|---|
| 1. Auth | Both devices sign in to Cognito and receive JWT tokens. |
| 2. API calls | All requests go through API Gateway with a Bearer token. API Gateway forwards to the Rust Lambda. Lambda validates the JWT, runs business logic, and reads/writes DynamoDB. |
| 3. File uploads | App requests a pre-signed URL from the Rust Lambda, then uploads the file directly to S3. Only the S3 object key is stored in DynamoDB. |

---

## 2. Request Flow — Create Cat with Photo Upload

Step-by-step HTTP calls when a user creates a cat and uploads a photo.

```mermaid
sequenceDiagram
    participant App as Mobile App
    participant Cognito as Cognito
    participant APIGW as API Gateway
    participant Lambda as Rust Lambda
    participant DDB as DynamoDB
    participant S3 as S3

    Note over App,S3: 1. Authentication
    App->>Cognito: Sign in (email + password)
    Cognito-->>App: JWT access token + refresh token

    Note over App,S3: 2. Create a cat
    App->>APIGW: POST /cats {name, breed, birthdate}\nAuthorization: Bearer token
    APIGW->>Lambda: Forward request
    Lambda->>Lambda: Validate JWT, extract ownerId
    Lambda->>DDB: PutItem to cats table
    DDB-->>Lambda: OK
    Lambda-->>App: 201 {data: {id, name, ...}}

    Note over App,S3: 3. Upload photo
    App->>APIGW: POST /uploads/presign {fileName, contentType}
    APIGW->>Lambda: Forward request
    Lambda->>S3: Generate pre-signed upload URL
    S3-->>Lambda: Signed URL + objectKey
    Lambda-->>App: {data: {uploadUrl, objectKey}}

    App->>S3: PUT photo to uploadUrl (direct upload)
    S3-->>App: 200 OK

    Note over App,S3: 4. Link photo to cat
    App->>APIGW: PUT /cats/{catId} {photoKey: objectKey}
    APIGW->>Lambda: Forward request
    Lambda->>DDB: UpdateItem, set photoKey
    DDB-->>Lambda: OK
    Lambda-->>App: 200 {data: {id, photoKey, ...}}

    Note over App,S3: 5. View cat photo later
    App->>APIGW: GET /files/{key}/url
    APIGW->>Lambda: Forward request
    Lambda->>S3: Generate pre-signed download URL
    S3-->>Lambda: Signed URL (15 min TTL)
    Lambda-->>App: {data: {downloadUrl}}
    App->>S3: GET photo from downloadUrl
    S3-->>App: Image data
```

---

## 3. Rust Backend — Internal Structure

How code is organized inside the single Rust Lambda.

```mermaid
graph LR
    subgraph RustLambda["Rust Lambda (axum)"]
        main["main.rs\nLambda entry + router"]

        subgraph MW["Middleware"]
            jwt["auth/middleware.rs\nJWT validation"]
            claims["auth/claims.rs\nToken claims struct"]
        end

        subgraph Routes["Route Handlers"]
            health["health.rs\nGET /health"]
            cats["cats.rs\nCRUD /cats"]
            meal["meal_reminders.rs\nCRUD meal reminders"]
            med["medicine_reminders.rs\nCRUD medicine reminders"]
            weight["weight_logs.rs\nCRUD weight logs"]
            hr["health_records.rs\nCRUD health records"]
            upload["uploads.rs\nPre-signed URLs"]
        end

        subgraph DB["DynamoDB Repos"]
            dc["cats_repo.rs"]
            dm["meal_repo.rs"]
            dmed["medicine_repo.rs"]
            dw["weight_repo.rs"]
            dh["health_repo.rs"]
        end

        subgraph Models["Models"]
            mc["cat.rs"]
            mm["meal_reminder.rs"]
            mmed["medicine_reminder.rs"]
            mw["weight_log.rs"]
            mh["health_record.rs"]
            resp["api_response.rs"]
        end

        s3mod["s3/mod.rs\nPre-sign helpers"]
        err["errors/mod.rs\nAppError enum"]
        cfg["config/mod.rs\nEnv vars"]
    end

    main --> MW
    MW --> Routes
    cats --> dc
    meal --> dm
    med --> dmed
    weight --> dw
    hr --> dh
    upload --> s3mod
    Routes --> err
    Routes --> Models
    DB --> cfg
```

### Module responsibilities

| Module | Responsibility |
|---|---|
| `main.rs` | Lambda entry point, initializes tracing, builds axum router, starts lambda_http runtime. |
| `auth/` | Validates Cognito JWT from the Authorization header. Extracts `ownerId` (the `sub` claim). |
| `routes/` | HTTP handlers. Parse request, call repo, return JSON response. |
| `db/` | DynamoDB access. One file per entity. All queries filter by `ownerId`. |
| `models/` | Rust structs for domain objects, create/update request bodies, and the API response envelope. |
| `errors/` | `AppError` enum with `IntoResponse` impl. Maps errors to HTTP status codes and the standard error envelope. |
| `config/` | Reads environment variables (table names, S3 bucket, Cognito pool ID). |
| `s3/` | Generates pre-signed upload and download URLs. |

---

## 4. Mobile App — Internal Structure

How the React Native code is organized.

```mermaid
graph TB
    subgraph MobileApp["React Native App"]
        Nav["RootNavigator.tsx"]

        subgraph AuthScreens["Auth"]
            SI["SignInScreen"]
            SU["SignUpScreen"]
        end

        subgraph AppScreens["App Screens"]
            Home["HomeScreen\n(Dashboard + Cat List)"]
            AddCat["AddCatScreen"]
            Profile["CatProfileScreen\n(Info + Weight Chart)"]
            MealR["MealReminderScreen"]
            MedR["MedicineReminderScreen"]
            WeightL["WeightLogScreen"]
            HealthR["HealthRecordsScreen"]
        end

        subgraph Services["Services"]
            API["apiClient.ts\n(axios + auth header)"]
            Auth["authService.ts\n(Cognito SDK)"]
            Notif["notificationService.ts\n(Notifee)"]
            Upload["uploadService.ts\n(S3 presign + upload)"]
        end

        subgraph Hooks["Data Hooks"]
            hCats["useCats()"]
            hRem["useReminders()"]
            hWeight["useWeightLogs()"]
            hHealth["useHealthRecords()"]
        end

        Types["types/index.ts"]
        Config["config/env.ts"]
    end

    Nav --> AuthScreens
    Nav --> AppScreens

    Home --> hCats
    Home --> hRem
    Profile --> hWeight
    HealthR --> hHealth

    hCats --> API
    hRem --> API
    hWeight --> API
    hHealth --> API

    API --> Auth
    API --> Config
    AddCat --> Upload
    AppScreens --> Notif
```

### Layer responsibilities

| Layer | Responsibility |
|---|---|
| **Navigator** | Routes users to auth screens (not signed in) or app screens (signed in). |
| **Screens** | UI for each feature. Calls hooks for data, renders components. |
| **Hooks** | `useCats()`, `useReminders()`, etc. Fetch data from the API, return `{ data, loading, error, refetch }`. |
| **Services** | Shared logic. `apiClient` injects auth token on every request. `authService` wraps Cognito SDK. `notificationService` schedules local reminders. `uploadService` handles S3 pre-sign + upload flow. |
| **Types** | All TypeScript interfaces in one file, matching the API contract. |
| **Config** | API base URL, Cognito IDs, S3 region. Never hardcoded in screens. |

---

## 5. Data Flow Between Devices

How Android and iPhone stay in sync without real-time subscriptions.

```mermaid
sequenceDiagram
    participant Android as Android
    participant API as Rust API
    participant DDB as DynamoDB
    participant iPhone as iPhone

    Android->>API: POST /cats (create "Mochi")
    API->>DDB: PutItem
    DDB-->>API: OK
    API-->>Android: 201 Created

    Note over Android,iPhone: iPhone does not know yet

    iPhone->>API: GET /cats (screen focus or pull-to-refresh)
    API->>DDB: Query by ownerId
    DDB-->>API: [Mochi, ...]
    API-->>iPhone: 200 {data: [Mochi, ...]}

    Note over Android,iPhone: Both devices now show Mochi
```

### Sync strategy (v1)

| Trigger | What happens |
|---|---|
| Screen mount | Fetch latest data from API. |
| Screen focus | Refetch via `useFocusEffect`. |
| After mutation | Refetch the affected list. |
| Pull-to-refresh | Manual refetch on list screens. |

This is sufficient for two devices and a small dataset. Real-time sync (WebSocket or polling) is optional for a later version.
