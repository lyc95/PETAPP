#!/bin/bash
# Populates the local PostgreSQL database with sample data for development.
# Creates a dev user and two cats with weight logs, health records, and medicine reminders.
# Run from anywhere inside the dev container:
#   bash .devcontainer/seed-local-data.sh

set -e

DB_URL="${DATABASE_URL:-postgres://catcare:dev@postgres:5432/catcare}"
NOW="2026-04-06 08:00:00+00"

psql() {
  command psql "$DB_URL" "$@"
}

echo "==> Seeding local PostgreSQL database..."
echo "    DB: $DB_URL"

psql <<SQL

-- -------------------------------------------------------------------------
-- Dev user  (password: "devpassword" hashed with argon2)
-- -------------------------------------------------------------------------
-- Pre-hashed value of "devpassword" via argon2id — avoids needing argon2 CLI.
-- The backend will accept this hash on POST /auth/login.
INSERT INTO users (id, email, password_hash, created_at)
VALUES (
  'aaaaaaaa-0000-0000-0000-000000000001',
  'dev@example.com',
  '\$argon2id\$v=19\$m=19456,t=2,p=1\$c29tZXNhbHRzb21lc2FsdA\$GdVCg6W8nMrMSHF5DkfFwRzCYwDn2WXLRVkOT5POYPY',
  '$NOW'
) ON CONFLICT (id) DO NOTHING;

-- -------------------------------------------------------------------------
-- Cats
-- -------------------------------------------------------------------------
INSERT INTO cats (id, owner_id, name, breed, birthdate, photo_key, created_at, updated_at)
VALUES
  ('a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'Mochi', 'Scottish Fold', '2021-03-15', NULL, '$NOW', '$NOW'),
  ('a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'Kuro', 'Domestic Shorthair', '2020-07-22', NULL, '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

-- -------------------------------------------------------------------------
-- Medicine reminders
-- -------------------------------------------------------------------------
INSERT INTO medicine_reminders
  (id, cat_id, owner_id, reminder_type, label, scheduled_date,
   is_recurring, interval_days, is_active, created_at, updated_at)
VALUES
  ('c1000001-0000-0000-0000-000000000001',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'MEDICATION', 'Flea & Tick Treatment', '2026-04-15 10:00:00+00',
   true, 30, true, '$NOW', '$NOW'),

  ('c2000002-0000-0000-0000-000000000001',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'NAIL_CUT', 'Nail Trimming', '2026-04-20 14:00:00+00',
   true, 14, true, '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

-- -------------------------------------------------------------------------
-- Weight logs — Mochi
-- -------------------------------------------------------------------------
INSERT INTO weight_logs (id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at)
VALUES
  ('d1000001-0000-0000-0000-000000000001',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   4.2, '2026-01-06 08:00:00+00', 'First weigh-in of the year', '$NOW', '$NOW'),
  ('d1000001-0000-0000-0000-000000000002',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   4.3, '2026-02-06 08:00:00+00', 'Slightly heavier', '$NOW', '$NOW'),
  ('d1000001-0000-0000-0000-000000000003',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   4.25, '2026-03-06 08:00:00+00', 'Back on track', '$NOW', '$NOW'),
  ('d1000001-0000-0000-0000-000000000004',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   4.4, '2026-04-06 08:00:00+00', 'Monthly check', '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

-- Weight logs — Kuro
INSERT INTO weight_logs (id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at)
VALUES
  ('d2000002-0000-0000-0000-000000000001',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   5.1, '2026-01-06 08:00:00+00', 'January check', '$NOW', '$NOW'),
  ('d2000002-0000-0000-0000-000000000002',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   5.0, '2026-02-06 08:00:00+00', 'Lost a bit', '$NOW', '$NOW'),
  ('d2000002-0000-0000-0000-000000000003',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   5.2, '2026-03-06 08:00:00+00', 'Back up', '$NOW', '$NOW'),
  ('d2000002-0000-0000-0000-000000000004',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   5.15, '2026-04-06 08:00:00+00', 'Stable', '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

-- -------------------------------------------------------------------------
-- Health records — Mochi
-- -------------------------------------------------------------------------
INSERT INTO health_records
  (id, cat_id, owner_id, record_type, title, description,
   recorded_at, attachment_key, created_at, updated_at)
VALUES
  ('e1000001-0000-0000-0000-000000000001',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'VACCINATION', 'Annual Vaccination',
   'FVRCP and rabies booster. All clear.',
   '2026-01-10 10:00:00+00', NULL, '$NOW', '$NOW'),

  ('e1000001-0000-0000-0000-000000000002',
   'a1b2c3d4-0001-0001-0001-000000000001',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'VET_VISIT', 'Check-up — mild cold',
   'Sneezing and runny nose. Prescribed antibiotics for 5 days.',
   '2026-03-02 14:30:00+00', NULL, '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

-- Health records — Kuro
INSERT INTO health_records
  (id, cat_id, owner_id, record_type, title, description,
   recorded_at, attachment_key, created_at, updated_at)
VALUES
  ('e2000002-0000-0000-0000-000000000001',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'VACCINATION', 'Annual Vaccination',
   'FVRCP booster. Healthy weight, good coat condition.',
   '2026-01-15 09:00:00+00', NULL, '$NOW', '$NOW'),

  ('e2000002-0000-0000-0000-000000000002',
   'a1b2c3d4-0002-0002-0002-000000000002',
   'aaaaaaaa-0000-0000-0000-000000000001',
   'NOTE', 'Prefers wet food',
   'Switched to wet food mix. Appetite improved significantly.',
   '2026-02-20 08:00:00+00', NULL, '$NOW', '$NOW')
ON CONFLICT (id) DO NOTHING;

SQL

echo ""
echo "==> Seed complete!"
echo "    Dev account:        dev@example.com / devpassword"
echo "    Cats:               Mochi (Scottish Fold), Kuro (Domestic Shorthair)"
echo "    Medicine reminders: 2 active reminders"
echo "    Weight logs:        4 entries per cat"
echo "    Health records:     2 entries per cat"
echo ""
echo "    Login: POST /auth/login"
echo '    Body:  {"email":"dev@example.com","password":"devpassword"}'
