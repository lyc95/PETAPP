#!/bin/bash
set -e

echo "==> Setting up Cat-Care dev environment..."

# Fix SSH key permissions (mounted from host — Windows FS sets them too open)
if [ -f /root/.ssh/id_ed25519 ]; then
  chmod 700 /root/.ssh
  chmod 600 /root/.ssh/id_ed25519
  echo "==> SSH key permissions fixed."
fi

# Install Claude Code CLI
echo "==> Installing Claude Code..."
npm install -g @anthropic-ai/claude-code

# Install mobile npm dependencies if package.json exists
if [ -f mobile/package.json ]; then
  echo "==> Installing mobile npm dependencies..."
  cd mobile && npm install && cd ..
fi

# Remind user to generate platform files if not yet done
if [ ! -d "mobile/android" ]; then
  echo ""
  echo "  NOTE: Android/iOS platform files not yet generated."
  echo "  Run once to create them: bash .devcontainer/init-mobile.sh"
fi

# Wait for PostgreSQL to be ready (docker-compose healthcheck handles this,
# but we wait here too in case post-create runs before the DB is fully up)
echo "==> Waiting for PostgreSQL to be ready..."
until pg_isready -h postgres -U catcare -d catcare &>/dev/null; do
  sleep 1
done
echo "==> PostgreSQL is ready."

# Migrations are applied automatically when cargo run starts, so no manual
# step is needed here. The backend will create all tables on first launch.

echo ""
echo "==> Dev environment ready!"
echo "    Backend:  cd backend && cargo run"
echo "    Mobile:   cd mobile  && npx react-native start"
echo ""
echo "    Backend API will be available at http://localhost:9000"
echo "    PostgreSQL: postgres://catcare:dev@localhost:5432/catcare"
