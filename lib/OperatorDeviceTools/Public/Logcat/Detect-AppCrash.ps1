function Detect-AppCrash {
    param(
        [Parameter(Mandatory=$true)]
        [string[]]$LogLines,
        [ValidateSet("Strict", "Warn")]
        [string]$Sensitivity = "Strict"
    )
    
    <#
    .SYNOPSIS
    Analyze logcat for crash patterns (SIGKILL, SIGSEGV, FATAL, etc.).
    
    .OUTPUTS
    [hashtable] with Crashed=$bool, Type=$string, Details=$string[]
    #>
    
    $crashPatterns = @(
        "FATAL EXCEPTION",
        "Force finishing activity",
        "SIGSEGV",
        "SIGKILL",
        "ANR in"
    )
    
    $details = @()
    $crashed = $false
    $type = "None"
    
    foreach ($line in $LogLines) {
        foreach ($pattern in $crashPatterns) {
            if ($line -match $pattern) {
                if (-not $crashed) {
                    $crashed = $true
                    $type = $pattern
                }
                $details += $line
            }
        }
    }
    
    if (-not $crashed -and $Sensitivity -eq "Warn") {
        $warnings = $LogLines | Where-Object { $_ -match " E/.*Exception" }
        if ($warnings) {
            $crashed = $true
            $type = "Unhandled Exception"
            $details = @($warnings)
        }
    }
    
    return @{
        Crashed = $crashed
        Type = $type
        Details = $details
    }
}
