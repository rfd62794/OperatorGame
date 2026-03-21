function Stop-OperatorApp {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [int]$TimeoutSeconds = 10
    )
    
    <#
    .SYNOPSIS
    Force stop OperatorGame.
    #>
    
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell am force-stop com.rfditservices.operatorgame" -NoErrorCheck | Out-Null
    
    $elapsed = 0
    while ((Is-AppRunning -Device $Device) -and ($elapsed -lt $TimeoutSeconds)) {
        Start-Sleep -Milliseconds 500
        $elapsed += 0.5
    }
    
    if (Is-AppRunning -Device $Device) {
        throw "Timeout: App com.rfditservices.operatorgame would not die after $TimeoutSeconds seconds."
    }
}
