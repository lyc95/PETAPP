#!/bin/bash
# Run this ONCE inside the devcontainer to generate Android/iOS platform files.
# Usage: bash .devcontainer/init-mobile.sh

set -e

if [ -d "mobile/android" ] && [ -d "mobile/ios" ]; then
  echo "Platform directories already exist. Nothing to do."
  exit 0
fi

echo "==> Generating React Native platform files (android/ + ios/)..."

# Init into a temp dir then copy platform dirs back
TMPDIR=$(mktemp -d)
npx react-native@0.74 init CatCare \
  --directory "$TMPDIR" \
  --skip-install \
  --pm npm

cp -r "$TMPDIR/android" mobile/android
cp -r "$TMPDIR/ios"     mobile/ios
rm -rf "$TMPDIR"

echo "==> Installing npm dependencies..."
cd mobile && npm install

echo ""
echo "==> Done! Platform files are ready."
echo "    Start Metro:   cd mobile && npx react-native start"
echo "    Run Android:   cd mobile && npx react-native run-android"
