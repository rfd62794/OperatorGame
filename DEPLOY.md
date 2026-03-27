# OperatorGame — Build & Deploy Guide

## Quick Start: Deploy to Moto G

### One-Command Deploy
```powershell
.\deploy_moto.ps1
```

This single command:
1. Sets up the build environment (Rust targets, NDK, Android SDK)
2. Builds and signs the APK via `cargo apk`
3. Installs the APK on your connected Moto G
4. Launches the app
5. Streams filtered logcat to your terminal

**Expected time:** 2–5 minutes for first build, ~60s for incremental.

---

## Prerequisites

| Requirement | How to get it |
|---|---|
| Moto G 2025 connected via USB | Physical USB cable |
| USB Debugging enabled | Settings → Developer Options → USB Debugging |
| Android Studio installed | [developer.android.com/studio](https://developer.android.com/studio) |
| NDK r25c installed | Android Studio → SDK Manager → SDK Tools → NDK (Side by side) |
| Rust toolchain | `rustup target add aarch64-linux-android armv7-linux-androideabi` |
| `cargo-apk` | `cargo install cargo-apk` |
| Release keystore | `.\build_android.ps1 -GenerateKeys` (one-time) |

---

## Scripts Reference

| Script | Purpose |
|---|---|
| `setup_local_forge.ps1` | Validate + configure build environment (Rust, NDK, SDK) |
| `build_android.ps1` | Build + sign APK for the Android release |
| `build_aab.ps1` | Build `.aab` bundle for Play Store submission |
| `check_moto.ps1` | Stream filtered logcat from connected device |
| `deploy_moto.ps1` | **Full pipeline** — build → install → launch → log |

---

## Manual Workflow

Run these steps individually if `deploy_moto.ps1` encounters an issue:

```powershell
# 1. Set up environment (once per terminal session)
#    Dot-source to preserve PATH/env changes:
. .\setup_local_forge.ps1

# 2. Confirm device is visible:
adb devices

# 3. Build and sign the APK (Interactive):
#    Note: This script will prompt you for your Keystore password.
.\build_android.ps1

# 4. Install on phone:
adb install -r operatorgame-release.apk

# 5. Launch the app:
adb shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"

# 6. Monitor logs (filtered to the app process):
.\check_moto.ps1
```

---

## Build Artifacts

| File | Description |
|---|---|
| `operatorgame-release.apk` | Signed, zipaligned APK — ready to install |
| `operatorgame-release.jks` | Release keystore — **back this up securely** |
| `target\googleplay\operatorgame-release-v*.aab` | Play Store bundle (run `build_aab.ps1`) |
| `bundletool.jar` | Auto-downloaded by `build_aab.ps1` — do not commit |

---

## Troubleshooting

| Error | Cause | Fix |
|---|---|---|
| `ADB not found` | SDK platform-tools missing or PATH wrong | Run `. .\setup_local_forge.ps1` |
| `No ADB device connected` | Phone not recognised | Check USB cable, re-enable USB Debugging, tap "Allow" on the USB auth dialog |
| `adb install FAILED` | Previous install corrupt | `adb uninstall com.rfditservices.operatorgame` then retry |
| `cargo apk build` fails | NDK linker not in PATH | Run `. .\setup_local_forge.ps1` — it injects NDK toolchain into session PATH |
| `Keystore not found` | First run, no keystore yet | `.\build_android.ps1 -GenerateKeys` |
| App launches to black screen | JVM crash or native panic | Run `.\check_moto.ps1` to read the error logcat |
| Missed `apksigner` | Build tools version mismatch | `setup_local_forge.ps1` picks the latest build-tools; verify in Android Studio |

---

## Play Store Submission

1. Run `.\build_aab.ps1` to produce a versioned `.aab`
2. Run the `jarsigner` command printed at the end of the script
3. Upload `target\googleplay\operatorgame-release-v*.aab` to Play Console

> **Note:** The version code and name are auto-read from `Cargo.toml` by `build_aab.ps1`.

---

## Environment Notes

**`.cargo/config.toml`** — Pins the NDK API-33 clang linker for all Android targets and sets the 16 KB page-size flag required by modern Android kernels (Android 15+).

**`Cargo.toml` `[package.metadata.android.signing.release]`** — Currently references the debug keystore. The PS1 build scripts re-sign the final APK with `operatorgame-release.jks`, so the deployed APK is correctly release-signed.
