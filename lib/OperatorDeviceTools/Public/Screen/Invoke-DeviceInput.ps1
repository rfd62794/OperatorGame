function Invoke-DeviceInput {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [string]$Text = $null,
        [int]$KeyCode = $null,
        [int]$DelayMs = 500
    )
    
    <#
    .SYNOPSIS
    Simulate standard keyboard strings or precise KeyCode injections on the device targeting focused inputs.
    #>
    
    if ($Text) {
        # Escape spaces for ADB shell string transfer
        $escapedText = $Text -replace ' ', '%s'
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell input text `"$escapedText`"" | Out-Null
    }
    
    if ($KeyCode -ne $null) {
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell input keyevent $KeyCode" | Out-Null
    }
    
    if ($DelayMs -gt 0) {
        Start-Sleep -Milliseconds $DelayMs
    }
}
