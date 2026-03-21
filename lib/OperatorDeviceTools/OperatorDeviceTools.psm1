# OperatorDeviceTools.psm1 — Unified Android Tools Subsystem

#region Module Configuration
$ModulePath = Split-Path -Parent $MyInvocation.MyCommand.Path
$PublicFunctions = Get-ChildItem -Path "$ModulePath\Public" -Filter "*.ps1" -Recurse -ErrorAction SilentlyContinue
$PrivateFunctions = Get-ChildItem -Path "$ModulePath\Private" -Filter "*.ps1" -Recurse -ErrorAction SilentlyContinue
$Classes = Get-ChildItem -Path "$ModulePath\Classes" -Filter "*.ps1" -Recurse -ErrorAction SilentlyContinue

# Load classes first
if ($Classes) {
    foreach ($Class in $Classes) {
        . $Class.FullName
    }
}

# Load private functions (not exported)
if ($PrivateFunctions) {
    foreach ($Function in $PrivateFunctions) {
        . $Function.FullName
    }
}

# Load public functions (exported)
if ($PublicFunctions) {
    foreach ($Function in $PublicFunctions) {
        . $Function.FullName
    }
}

# Define module properties
$Script:OperatorDeviceTools = @{
    AdbPath         = $null
    SdkPath         = $null
    RepositoryRoot  = (Get-Item (Split-Path -Parent $ModulePath)).Parent.FullName
    LogLevel        = "Info"  # Debug, Info, Warn, Error
    DefaultSerial   = $null
    SessionTimeout  = 300     # seconds
}

# Export public functions robustly
if ($PublicFunctions) {
    $PublicFunctionNames = $PublicFunctions | ForEach-Object { [System.IO.Path]::GetFileNameWithoutExtension($_.Name) }
    Export-ModuleMember -Function $PublicFunctionNames
}
#endregion
