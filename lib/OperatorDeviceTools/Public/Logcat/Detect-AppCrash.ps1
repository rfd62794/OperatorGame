function Detect-AppCrash {
    param(
        [Parameter(Mandatory=$true)]
        [string[]]$LogLines
    )
    
    <#
    .SYNOPSIS
    Analyze logcat buffers for native vs JVM crash signatures.
    #>
    
    $nativeCrashPatterns = @("SIGSEGV", "SIGKILL", "dlopen failed", "linker error", "library .* not found")
    $javaCrashPatterns = @("FATAL EXCEPTION", "Force finishing activity", "ANR in")
    
    $details = @()
    $crashed = $false
    $type = "None"
    
    foreach ($line in $LogLines) {
        foreach ($p in $nativeCrashPatterns) {
            if ($line -match $p) {
                if (-not $crashed) { $type = "NDK Native Crash ($p)" }
                $crashed = $true
                $details += $line
            }
        }
        foreach ($p in $javaCrashPatterns) {
            if ($line -match $p) {
                if (-not $crashed) { $type = "Java Exception ($p)" }
                $crashed = $true
                $details += $line
            }
        }
    }
    
    return @{
        Crashed = $crashed
        Type = $type
        Details = $details
    }
}
