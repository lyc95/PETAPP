#!/bin/bash
# Run this ONCE inside the devcontainer to generate Android/iOS platform files.
# Usage: bash .devcontainer/init-mobile.sh

set -e

if [ -d "mobile/android" ] && [ -d "mobile/ios" ]; then
  echo "Platform directories already exist. Nothing to do."
  exit 0
fi

echo "==> Generating React Native platform files (android/ + ios/)..."

# Stub out 'pod' so RN CLI doesn't fail on Linux (CocoaPods is macOS-only).
# Without this, the CLI treats the pod failure as fatal and rolls back the
# entire template copy — leaving no android/ directory at all.
MOCK_BIN=$(mktemp -d)
printf '#!/bin/sh\nexit 0\n' > "$MOCK_BIN/pod"
chmod +x "$MOCK_BIN/pod"
export PATH="$MOCK_BIN:$PATH"

# Init into a temp dir then copy platform dirs back
TMPDIR=$(mktemp -d)
npx react-native@0.79 init CatCare \
  --directory "$TMPDIR" \
  --skip-install \
  --pm npm

cp -r "$TMPDIR/android" mobile/android
cp -r "$TMPDIR/ios"     mobile/ios
rm -rf "$TMPDIR" "$MOCK_BIN"

echo "==> Installing npm dependencies..."
cd mobile && npm install

echo ""
echo "==> Done! Platform files are ready."
echo "    Start Metro:   cd mobile && npx react-native start"
echo "    Run Android:   cd mobile && npx react-native run-android"
