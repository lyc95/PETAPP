CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    email         TEXT        NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE cats (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id   UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name       TEXT        NOT NULL,
    breed      TEXT        NOT NULL,
    birthdate  DATE        NOT NULL,
    photo_key  TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX cats_owner_idx ON cats(owner_id);

CREATE TABLE weight_logs (
    id         UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    cat_id     UUID         NOT NULL REFERENCES cats(id) ON DELETE CASCADE,
    owner_id   UUID         NOT NULL REFERENCES users(id),
    weight_kg  FLOAT8 NOT NULL,
    logged_at  TIMESTAMPTZ  NOT NULL,
    note       TEXT,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now()
);
CREATE INDEX weight_logs_cat_idx ON weight_logs(cat_id, logged_at DESC);

CREATE TABLE health_records (
    id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    cat_id         UUID        NOT NULL REFERENCES cats(id) ON DELETE CASCADE,
    owner_id       UUID        NOT NULL REFERENCES users(id),
    record_type    TEXT        NOT NULL,
    title          TEXT        NOT NULL,
    description    TEXT        NOT NULL DEFAULT '',
    recorded_at    TIMESTAMPTZ NOT NULL,
    attachment_key TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX health_records_cat_idx ON health_records(cat_id, recorded_at DESC);

CREATE TABLE medicine_reminders (
    id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    cat_id         UUID        NOT NULL REFERENCES cats(id) ON DELETE CASCADE,
    owner_id       UUID        NOT NULL REFERENCES users(id),
    reminder_type  TEXT        NOT NULL,
    label          TEXT        NOT NULL,
    scheduled_date TIMESTAMPTZ NOT NULL,
    is_recurring   BOOLEAN     NOT NULL DEFAULT false,
    interval_days  INT,
    is_active      BOOLEAN     NOT NULL DEFAULT true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX medicine_reminders_cat_idx ON medicine_reminders(cat_id);
