#!/bin/bash
# Populates local DynamoDB tables with sample data for local development.
# Owner ID matches the LOCAL_MODE default in the mobile app: "local-dev-user"
# Run from anywhere inside the dev container:
#   bash .devcontainer/seed-local-data.sh

set -e

ENDPOINT="http://dynamodb:8000"
REGION="ap-southeast-1"
STAGE="${STAGE:-dev}"
OWNER="local-dev-user"
NOW="2026-04-06T08:00:00Z"

aws_local() {
  aws dynamodb "$@" --endpoint-url "$ENDPOINT" --region "$REGION"
}

echo "==> Seeding local DynamoDB tables (stage: $STAGE, owner: $OWNER)..."

# ---------------------------------------------------------------------------
# Cats
# ---------------------------------------------------------------------------
CATS_TABLE="catcare-cats-${STAGE}"

cat_mochi="a1b2c3d4-0001-0001-0001-000000000001"
cat_kuro="a1b2c3d4-0002-0002-0002-000000000002"

echo "  [cats] Mochi..."
aws_local put-item --table-name "$CATS_TABLE" --item '{
  "id":        {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":   {"S": "local-dev-user"},
  "name":      {"S": "Mochi"},
  "breed":     {"S": "Scottish Fold"},
  "birthdate": {"S": "2021-03-15"},
  "photoKey":  {"NULL": true},
  "createdAt": {"S": "2026-04-06T08:00:00Z"},
  "updatedAt": {"S": "2026-04-06T08:00:00Z"}
}'

echo "  [cats] Kuro..."
aws_local put-item --table-name "$CATS_TABLE" --item '{
  "id":        {"S": "a1b2c3d4-0002-0002-0002-000000000002"},
  "ownerId":   {"S": "local-dev-user"},
  "name":      {"S": "Kuro"},
  "breed":     {"S": "Domestic Shorthair"},
  "birthdate": {"S": "2020-07-22"},
  "photoKey":  {"NULL": true},
  "createdAt": {"S": "2026-04-06T08:00:00Z"},
  "updatedAt": {"S": "2026-04-06T08:00:00Z"}
}'

# ---------------------------------------------------------------------------
# Meal reminders
# ---------------------------------------------------------------------------
MEAL_TABLE="catcare-meal-reminders-${STAGE}"

echo "  [meal reminders] Mochi breakfast..."
aws_local put-item --table-name "$MEAL_TABLE" --item '{
  "id":            {"S": "b1000001-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":       {"S": "local-dev-user"},
  "label":         {"S": "Breakfast"},
  "scheduledTime": {"S": "07:30"},
  "daysOfWeek":    {"L": [{"N":"1"},{"N":"2"},{"N":"3"},{"N":"4"},{"N":"5"}]},
  "isActive":      {"BOOL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

echo "  [meal reminders] Mochi dinner..."
aws_local put-item --table-name "$MEAL_TABLE" --item '{
  "id":            {"S": "b1000001-0000-0000-0000-000000000002"},
  "catId":         {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":       {"S": "local-dev-user"},
  "label":         {"S": "Dinner"},
  "scheduledTime": {"S": "18:00"},
  "daysOfWeek":    {"L": [{"N":"0"},{"N":"1"},{"N":"2"},{"N":"3"},{"N":"4"},{"N":"5"},{"N":"6"}]},
  "isActive":      {"BOOL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

echo "  [meal reminders] Kuro breakfast..."
aws_local put-item --table-name "$MEAL_TABLE" --item '{
  "id":            {"S": "b2000002-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0002-0002-0002-000000000002"},
  "ownerId":       {"S": "local-dev-user"},
  "label":         {"S": "Breakfast"},
  "scheduledTime": {"S": "08:00"},
  "daysOfWeek":    {"L": [{"N":"0"},{"N":"1"},{"N":"2"},{"N":"3"},{"N":"4"},{"N":"5"},{"N":"6"}]},
  "isActive":      {"BOOL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

# ---------------------------------------------------------------------------
# Medicine reminders
# ---------------------------------------------------------------------------
MED_TABLE="catcare-medicine-reminders-${STAGE}"

echo "  [medicine reminders] Mochi flea treatment..."
aws_local put-item --table-name "$MED_TABLE" --item '{
  "id":            {"S": "c1000001-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":       {"S": "local-dev-user"},
  "reminderType":  {"S": "MEDICATION"},
  "label":         {"S": "Flea & Tick Treatment"},
  "scheduledDate": {"S": "2026-04-15T10:00:00Z"},
  "isRecurring":   {"BOOL": true},
  "intervalDays":  {"N": "30"},
  "isActive":      {"BOOL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

echo "  [medicine reminders] Kuro nail cut..."
aws_local put-item --table-name "$MED_TABLE" --item '{
  "id":            {"S": "c2000002-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0002-0002-0002-000000000002"},
  "ownerId":       {"S": "local-dev-user"},
  "reminderType":  {"S": "NAIL_CUT"},
  "label":         {"S": "Nail Trimming"},
  "scheduledDate": {"S": "2026-04-20T14:00:00Z"},
  "isRecurring":   {"BOOL": true},
  "intervalDays":  {"N": "14"},
  "isActive":      {"BOOL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

# ---------------------------------------------------------------------------
# Weight logs
# ---------------------------------------------------------------------------
WEIGHT_TABLE="catcare-weight-logs-${STAGE}"

echo "  [weight logs] Mochi..."
for entry in \
  "d1000001-0000-0000-0000-000000000001|4.2|2026-01-06T08:00:00Z|First weigh-in of the year" \
  "d1000001-0000-0000-0000-000000000002|4.3|2026-02-06T08:00:00Z|Slightly heavier" \
  "d1000001-0000-0000-0000-000000000003|4.25|2026-03-06T08:00:00Z|Back on track" \
  "d1000001-0000-0000-0000-000000000004|4.4|2026-04-06T08:00:00Z|Monthly check"; do
  IFS='|' read -r id kg logged_at note <<< "$entry"
  aws_local put-item --table-name "$WEIGHT_TABLE" --item "{
    \"id\":        {\"S\": \"$id\"},
    \"catId\":     {\"S\": \"a1b2c3d4-0001-0001-0001-000000000001\"},
    \"ownerId\":   {\"S\": \"local-dev-user\"},
    \"weightKg\":  {\"N\": \"$kg\"},
    \"loggedAt\":  {\"S\": \"$logged_at\"},
    \"note\":      {\"S\": \"$note\"},
    \"createdAt\": {\"S\": \"$NOW\"},
    \"updatedAt\": {\"S\": \"$NOW\"}
  }"
done

echo "  [weight logs] Kuro..."
for entry in \
  "d2000002-0000-0000-0000-000000000001|5.1|2026-01-06T08:00:00Z|January check" \
  "d2000002-0000-0000-0000-000000000002|5.0|2026-02-06T08:00:00Z|Lost a bit" \
  "d2000002-0000-0000-0000-000000000003|5.2|2026-03-06T08:00:00Z|Back up" \
  "d2000002-0000-0000-0000-000000000004|5.15|2026-04-06T08:00:00Z|Stable"; do
  IFS='|' read -r id kg logged_at note <<< "$entry"
  aws_local put-item --table-name "$WEIGHT_TABLE" --item "{
    \"id\":        {\"S\": \"$id\"},
    \"catId\":     {\"S\": \"a1b2c3d4-0002-0002-0002-000000000002\"},
    \"ownerId\":   {\"S\": \"local-dev-user\"},
    \"weightKg\":  {\"N\": \"$kg\"},
    \"loggedAt\":  {\"S\": \"$logged_at\"},
    \"note\":      {\"S\": \"$note\"},
    \"createdAt\": {\"S\": \"$NOW\"},
    \"updatedAt\": {\"S\": \"$NOW\"}
  }"
done

# ---------------------------------------------------------------------------
# Health records
# ---------------------------------------------------------------------------
HEALTH_TABLE="catcare-health-records-${STAGE}"

echo "  [health records] Mochi..."
aws_local put-item --table-name "$HEALTH_TABLE" --item '{
  "id":            {"S": "e1000001-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":       {"S": "local-dev-user"},
  "recordType":    {"S": "VACCINATION"},
  "title":         {"S": "Annual Vaccination"},
  "description":   {"S": "FVRCP and rabies booster. All clear."},
  "recordedAt":    {"S": "2026-01-10T10:00:00Z"},
  "attachmentKey": {"NULL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

aws_local put-item --table-name "$HEALTH_TABLE" --item '{
  "id":            {"S": "e1000001-0000-0000-0000-000000000002"},
  "catId":         {"S": "a1b2c3d4-0001-0001-0001-000000000001"},
  "ownerId":       {"S": "local-dev-user"},
  "recordType":    {"S": "VET_VISIT"},
  "title":         {"S": "Check-up — mild cold"},
  "description":   {"S": "Sneezing and runny nose. Prescribed antibiotics for 5 days."},
  "recordedAt":    {"S": "2026-03-02T14:30:00Z"},
  "attachmentKey": {"NULL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

echo "  [health records] Kuro..."
aws_local put-item --table-name "$HEALTH_TABLE" --item '{
  "id":            {"S": "e2000002-0000-0000-0000-000000000001"},
  "catId":         {"S": "a1b2c3d4-0002-0002-0002-000000000002"},
  "ownerId":       {"S": "local-dev-user"},
  "recordType":    {"S": "VACCINATION"},
  "title":         {"S": "Annual Vaccination"},
  "description":   {"S": "FVRCP booster. Healthy weight, good coat condition."},
  "recordedAt":    {"S": "2026-01-15T09:00:00Z"},
  "attachmentKey": {"NULL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

aws_local put-item --table-name "$HEALTH_TABLE" --item '{
  "id":            {"S": "e2000002-0000-0000-0000-000000000002"},
  "catId":         {"S": "a1b2c3d4-0002-0002-0002-000000000002"},
  "ownerId":       {"S": "local-dev-user"},
  "recordType":    {"S": "NOTE"},
  "title":         {"S": "Prefers wet food"},
  "description":   {"S": "Switched to wet food mix. Appetite improved significantly."},
  "recordedAt":    {"S": "2026-02-20T08:00:00Z"},
  "attachmentKey": {"NULL": true},
  "createdAt":     {"S": "2026-04-06T08:00:00Z"},
  "updatedAt":     {"S": "2026-04-06T08:00:00Z"}
}'

echo ""
echo "==> Seed complete!"
echo "    Cats:              Mochi (Scottish Fold), Kuro (Domestic Shorthair)"
echo "    Meal reminders:    3 active reminders"
echo "    Medicine reminders: 2 active reminders"
echo "    Weight logs:       4 entries per cat"
echo "    Health records:    2 entries per cat"
