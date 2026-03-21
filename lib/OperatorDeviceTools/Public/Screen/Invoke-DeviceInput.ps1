function Invoke-DeviceInput {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [string]$Text = $null,
        [string]$KeyPhrase = $null,
        [int]$KeyCode = $null,
        [int]$DelayMs = 500
    )
    
    <#
    .SYNOPSIS
    Simulate standard keyboard strings or precise KeyCode injections on the device targeting focused inputs.
    #>
    
    $keyCodes = @{
        "HOME" = 3
        "BACK" = 4
        "ENTER" = 66
        "DEL" = 67
        "TAB" = 61
        "SPACE" = 62
    }
    
    if ($KeyPhrase) {
        $upperKey = $KeyPhrase.ToUpper()
        if ($keyCodes.ContainsKey($upperKey)) {
            $KeyCode = $keyCodes[$upperKey]
        }
    }
    
    if ($Text) {
        # Escape spaces for ADB shell string transfer
        $escapedText = $Text -replace '\s', '%s'
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell input text `"$escapedText`"" | Out-Null
    }
    
    if ($KeyCode -ne $null) {
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell input keyevent $KeyCode" | Out-Null
    }
    
    if ($DelayMs -gt 0) {
        Start-Sleep -Milliseconds $DelayMs
    }
}
