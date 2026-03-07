#!/usr/bin/env bash
# Builds, packages, aligns, and signs the Android APK for Play Store deployment

set -e

KEYSTORE="operatorgame-release.jks"
ALIAS="operatorgame"
APK_UNSIGNED="target/aarch64-linux-android/release/apk/operator.apk" # Adjust based on cargo-apk output
APK_ALIGNED="operatorgame-release-aligned.apk"
APK_FINAL="operatorgame-release.apk"

if [ "$1" == "--generate-keys" ]; then
    echo "Generating release keystore..."
    keytool -genkey -v \
      -keystore "$KEYSTORE" \
      -alias "$ALIAS" \
      -keyalg RSA -keysize 2048 \
      -validity 10000
    echo "⚠️ IMPORTANT: Backup $KEYSTORE securely in 2+ locations! ⚠️"
    exit 0
fi

if [ ! -f "$KEYSTORE" ]; then
    echo "Error: Keystore not found at $KEYSTORE"
    echo "Run './build_android.sh --generate-keys' first to create one."
    exit 1
fi

echo "🚀 Building release binary (aarch64-linux-android)..."
cargo build --release --target aarch64-linux-android

echo "📦 Packaging APK..."
# Replace with xbuild if your project uses it instead of cargo-apk
cargo apk build --release 

if [ ! -f "$APK_UNSIGNED" ]; then
    echo "Error: Could not find output APK at $APK_UNSIGNED. Check build pathway."
    exit 1
fi

echo "🔐 Aligning APK..."
zipalign -v -p 4 "$APK_UNSIGNED" "$APK_ALIGNED"

echo "✍️ Signing APK via apksigner (or jarsigner fallback)..."
if command -v apksigner &> /dev/null; then
    apksigner sign --ks "$KEYSTORE" --out "$APK_FINAL" "$APK_ALIGNED"
else
    echo "apksigner not found, falling back to jarsigner..."
    cp "$APK_ALIGNED" "$APK_FINAL"
    jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 -keystore "$KEYSTORE" "$APK_FINAL" "$ALIAS"
fi

rm "$APK_ALIGNED"

echo "✅ Success! Final signed APK ready at: $APK_FINAL"
