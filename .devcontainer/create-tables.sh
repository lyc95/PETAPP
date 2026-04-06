#!/bin/bash
# Creates all DynamoDB Local tables for local development.
# Mirrors the SAM template table definitions (dev stage).

ENDPOINT="http://dynamodb:8000"
REGION="ap-southeast-1"
STAGE="${STAGE:-dev}"

aws_local() {
  aws dynamodb "$@" --endpoint-url "$ENDPOINT" --region "$REGION"
}

table_exists() {
  aws_local describe-table --table-name "$1" &>/dev/null
}

echo "Creating tables for stage: $STAGE"

# catcare-cats-{stage}
TABLE="catcare-cats-${STAGE}"
if ! table_exists "$TABLE"; then
  aws_local create-table \
    --table-name "$TABLE" \
    --attribute-definitions \
      AttributeName=id,AttributeType=S \
      AttributeName=ownerId,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --global-secondary-indexes '[
      {"IndexName":"ownerId-index","KeySchema":[{"AttributeName":"ownerId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}}
    ]'
  echo "  Created $TABLE"
else
  echo "  Skipped $TABLE (already exists)"
fi

# catcare-meal-reminders-{stage}
TABLE="catcare-meal-reminders-${STAGE}"
if ! table_exists "$TABLE"; then
  aws_local create-table \
    --table-name "$TABLE" \
    --attribute-definitions \
      AttributeName=id,AttributeType=S \
      AttributeName=catId,AttributeType=S \
      AttributeName=ownerId,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --global-secondary-indexes '[
      {"IndexName":"catId-index","KeySchema":[{"AttributeName":"catId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}},
      {"IndexName":"ownerId-index","KeySchema":[{"AttributeName":"ownerId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}}
    ]'
  echo "  Created $TABLE"
else
  echo "  Skipped $TABLE (already exists)"
fi

# catcare-medicine-reminders-{stage}
TABLE="catcare-medicine-reminders-${STAGE}"
if ! table_exists "$TABLE"; then
  aws_local create-table \
    --table-name "$TABLE" \
    --attribute-definitions \
      AttributeName=id,AttributeType=S \
      AttributeName=catId,AttributeType=S \
      AttributeName=ownerId,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --global-secondary-indexes '[
      {"IndexName":"catId-index","KeySchema":[{"AttributeName":"catId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}},
      {"IndexName":"ownerId-index","KeySchema":[{"AttributeName":"ownerId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}}
    ]'
  echo "  Created $TABLE"
else
  echo "  Skipped $TABLE (already exists)"
fi

# catcare-weight-logs-{stage}
TABLE="catcare-weight-logs-${STAGE}"
if ! table_exists "$TABLE"; then
  aws_local create-table \
    --table-name "$TABLE" \
    --attribute-definitions \
      AttributeName=id,AttributeType=S \
      AttributeName=catId,AttributeType=S \
      AttributeName=loggedAt,AttributeType=S \
      AttributeName=ownerId,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --global-secondary-indexes '[
      {"IndexName":"catId-loggedAt-index","KeySchema":[{"AttributeName":"catId","KeyType":"HASH"},{"AttributeName":"loggedAt","KeyType":"RANGE"}],"Projection":{"ProjectionType":"ALL"}},
      {"IndexName":"ownerId-index","KeySchema":[{"AttributeName":"ownerId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}}
    ]'
  echo "  Created $TABLE"
else
  echo "  Skipped $TABLE (already exists)"
fi

# catcare-health-records-{stage}
TABLE="catcare-health-records-${STAGE}"
if ! table_exists "$TABLE"; then
  aws_local create-table \
    --table-name "$TABLE" \
    --attribute-definitions \
      AttributeName=id,AttributeType=S \
      AttributeName=catId,AttributeType=S \
      AttributeName=recordedAt,AttributeType=S \
      AttributeName=ownerId,AttributeType=S \
    --key-schema AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --global-secondary-indexes '[
      {"IndexName":"catId-recordedAt-index","KeySchema":[{"AttributeName":"catId","KeyType":"HASH"},{"AttributeName":"recordedAt","KeyType":"RANGE"}],"Projection":{"ProjectionType":"ALL"}},
      {"IndexName":"ownerId-index","KeySchema":[{"AttributeName":"ownerId","KeyType":"HASH"}],"Projection":{"ProjectionType":"ALL"}}
    ]'
  echo "  Created $TABLE"
else
  echo "  Skipped $TABLE (already exists)"
fi

echo "Done."
