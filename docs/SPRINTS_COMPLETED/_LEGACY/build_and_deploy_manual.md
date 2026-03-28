# Project Infrastructure: Android Build & Deployment System

This document describes the automated pipeline used to build, sign, and deploy the `OperatorGame` (Rust/egui) application to physical Android hardware.

## Core Identification
- **Package Name**: `com.rfditservices.operatorgame`
- **Main Technology**: Rust + `egui` + `cargo-apk`
- **Native Activity**: Android `NativeActivity` (handled by `cargo-apk`)

## 1. Build Phase (`build_android.ps1`)
The build process is encapsulated in a PowerShell script at the repository root.

- **Environment Requirements**:
    - Android SDK with `build-tools` (zipalign, apksigner).
    - Rust with `cargo-apk` installed.
    - Local Keystore: `operatorgame-release.jks` (Alias: `operatorgame`).
- **Workflow**:
    1. Runs `cargo apk build --release --lib`.
    2. Discovers the generated unsigned APK in `target/release/apk/`.
    3. Runs `zipalign -v -p 4` on the artifact.
    4. Signs the aligned APK using `apksigner` (or `jarsigner` if apksigner is missing).
    5. Outputs the final production-ready artifact: `operatorgame-release.apk`.

## 2. Deployment Phase (`deploy_moto.ps1`)
The deployment script automates side-loading and bootstrapping the engine on a connected device.

- **Workflow**:
    1. Imports the `OperatorDeviceTools` module.
    2. **Connect**: Validates ADB connection to the target hardware.
    3. **Install**: Sideloads the latest `operatorgame-release.apk` using `adb install -r`.
    4. **Launch**: Triggers the engine bootstrap. It uses the `monkey` tool for resilient launching:
       `adb shell monkey -p com.rfditservices.operatorgame -c android.intent.category.LAUNCHER 1`
    5. **Verify**: Polls for the native PID to confirm the app has successfully entered the foreground.

## 3. Automation Subsystem (`lib/OperatorDeviceTools/`)
A modular PowerShell toolset for device interaction.

- **`Connect-Device.ps1`**: Utility to scan for and lock onto an Android device via ADB.
- **`Install-OperatorApp.ps1`**: Auto-discovers the latest APK in the workspace and handles the `adb install` lifecycle.
- **`Launch-OperatorApp.ps1`**: Manages app startup, including killing stale instances and verifying process bootstrap.
- **`Logcat` Utility**: (Associated tools) allow for real-time monitoring of the Rust `tracing` output on-device.

## Usage
To rebuild and deploy after a change:
1. Run `.\build_android.ps1` to generate the APK.
2. Ensure the phone is connected via USB/ADB.
3. Run `.\deploy_moto.ps1` to install and launch.
