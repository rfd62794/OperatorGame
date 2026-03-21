function Is-AppRunning {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device
    )
    
    <#
    .SYNOPSIS
    Check if OperatorGame is currently running.
    
    .DESCRIPTION
    Executes 'pidof' to check the memory residency of the game.
    Safely captures errors and parses output dynamically.
    
    .OUTPUTS
    [int] PID if running, $null if not
    #>
    
    $pidRaw = Invoke-AdbCommand -Serial $Device.Serial -Command "shell pidof com.rfditservices.operatorgame" -NoErrorCheck
    if ($pidRaw -and -not $pidRaw.StartsWith("Error")) {
        $trimmed = $pidRaw.Trim()
        if ($trimmed) {
            # In case multiple PIDs are returned, grab the first one
            return [int]($trimmed -split '\s+')[0]
        }
    }
    return $null
}
