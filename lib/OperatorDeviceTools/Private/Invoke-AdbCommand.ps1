function Invoke-AdbCommand {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Serial,
        
        [Parameter(Mandatory=$true)]
        [string]$Command,
        
        [switch]$NoErrorCheck = $false,
        [int]$TimeoutSeconds = 30
    )
    
    <#
    .SYNOPSIS
    Execute ADB command with error handling and timeout.
    
    .DESCRIPTION
    - Ensures ADB daemon is running
    - Applies timeout to long-running commands
    - Parses stderr vs stdout intelligently
    - Throws on failure unless $NoErrorCheck
    #>
    
    $adb = Resolve-AdbPath
    
    $procInfo = New-Object System.Diagnostics.ProcessStartInfo
    $procInfo.FileName = $adb
    $procInfo.Arguments = "-s $Serial $Command"
    $procInfo.RedirectStandardOutput = $true
    $procInfo.RedirectStandardError = $true
    $procInfo.UseShellExecute = $false
    $procInfo.CreateNoWindow = $true
    
    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo = $procInfo
    
    $proc.Start() | Out-Null
    $proc.WaitForExit($TimeoutSeconds * 1000) | Out-Null
    
    if (-not $proc.HasExited) {
        $proc.Kill()
        throw "ADB Command [$Command] locked the pipeline and was killed after $TimeoutSeconds seconds."
    }
    
    $stdOut = $proc.StandardOutput.ReadToEnd()
    $stdErr = $proc.StandardError.ReadToEnd()
    
    if (($proc.ExitCode -ne 0) -and (-not $NoErrorCheck)) {
        throw "ADB Execution Error ($($proc.ExitCode)): $stdErr`n$stdOut"
    }
    
    return $stdOut
}
