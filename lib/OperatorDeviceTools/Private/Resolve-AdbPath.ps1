function Resolve-AdbPath {
    if ($Script:OperatorDeviceTools.AdbPath) { return $Script:OperatorDeviceTools.AdbPath }
    
    $default = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    if (Test-Path $default) {
        $Script:OperatorDeviceTools.AdbPath = $default
        return $default
    }
    
    $inPath = Get-Command "adb" -ErrorAction SilentlyContinue
    if ($inPath) {
        $Script:OperatorDeviceTools.AdbPath = $inPath.Source
        return $inPath.Source
    }
    
    throw "ADB executable not found. Please verify Android SDK is installed."
}
