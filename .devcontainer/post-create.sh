#!/bin/bash
set -e

echo "==> Setting up Cat-Care dev environment..."

# Install mobile npm dependencies if package.json exists
if [ -f mobile/package.json ]; then
  echo "==> Installing mobile npm dependencies..."
  cd mobile && npm install && cd ..
fi

# Create local DynamoDB tables
echo "==> Waiting for DynamoDB Local to be ready..."
until aws dynamodb list-tables --endpoint-url http://dynamodb:8000 --region us-east-1 &>/dev/null; do
  sleep 1
done

echo "==> Creating DynamoDB tables..."
bash .devcontainer/create-tables.sh

echo ""
echo "==> Dev environment ready!"
echo "    Backend:  cd backend && cargo lambda watch"
echo "    Mobile:   cd mobile  && npx react-native start"
