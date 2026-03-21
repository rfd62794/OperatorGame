# OperatorGame Mobile QA Subsystem Architecture
**Phase:** SDD - Foundation & Architectural Mapping

## Executive Summary
The ad-hoc scripting approach has hit the limits of complexity. Both `capture_screenshots.ps1` and `find_ui_coordinates.ps1` are heavily duplicating initialization, detection, and validation logic. We must pivot to a modular architecture to enforce DRY (Don't Repeat Yourself) principles and inject robust lifecycle boundaries (like auto-installing missing APKs). 

## Proposed Architecture (PowerShell Module)

### 1. `OperatorDeviceTools.psm1` (Core Dependency)
A standardized library encapsulating all inter-process operations via ADB.
- **`Connect-Device`**: Auto-detects Moto G device arrays safely and enforces locking.
- **`Install-OperatorApp`**: Validates against `pm list`. If missing, parses the project workspace for an `.apk` and forces an `adb install`.
- **`Launch-OperatorApp`**: Manages the `monkey` intent lifecycle and enforces async delays.
- **`Capture-DeviceScreen`**: Secure pipeline ignoring output stream corruption.
- **`Invoke-DeviceTap`**: Abstracts the coordinate inputs.

### 2. `find_ui_coordinates.ps1` (Telemetry Layer)
Imports `OperatorDeviceTools.psm1`.
Focuses purely on managing the user input loop, background `getevent` sniffing jobs, and hex conversion mappings.

### 3. `capture_screenshots.ps1` (QA Driver)
Imports `OperatorDeviceTools.psm1`.
Focuses completely on orchestrating the declarative steps for the Roster, Missions, Map, and Logs visual flow.
