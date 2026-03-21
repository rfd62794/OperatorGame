function Parse-LogcatBuffer {
    param(
        [Parameter(Mandatory=$true)]
        [string[]]$LogLines,
        [string]$SeverityPattern = "[EF]/", # Errors and Fatals
        [string]$Keyword = $null
    )
    
    <#
    .SYNOPSIS
    Filter logcat buffers offline.
    #>
    
    $results = @()
    foreach ($line in $LogLines) {
        if ($line -match $SeverityPattern) {
            if (-not $Keyword -or $line -match $Keyword) {
                $results += $line
            }
        }
    }
    return $results
}
