function Stop-OperatorApp {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device
    )
    
    <#
    .SYNOPSIS
    Force stop OperatorGame.
    #>
    
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell am force-stop com.rfditservices.operatorgame" -NoErrorCheck | Out-Null
}
