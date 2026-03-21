function Parse-LogcatBuffer {
    param(
        [Parameter(Mandatory=$true)]
        [string[]]$LogLines,
        [string]$FilterPackage = "com.rfditservices.operatorgame",
        [string]$Severity = $null,
        [string]$Pid = $null
    )
    
    <#
    .SYNOPSIS
    Filter offline logcat arrays into strongly typed [PSCustomObject] metrics.
    #>
    
    $results = @()
    # Logcat line structural mapping
    $regex = '^(?<Date>\d{2}-\d{2})\s+(?<Time>\d{2}:\d{2}:\d{2}\.\d{3})\s+(?<PID>\d+)\s+(?<TID>\d+)\s+(?<Level>[VDIWEFS])\s+(?<Tag>.*?):\s+(?<Message>.*)$'
    
    foreach ($line in $LogLines) {
        if ($line -match $regex) {
            $parsedPid = $matches['PID']
            $parsedLevel = $matches['Level']
            
            if ($Pid -and $parsedPid -ne $Pid) { continue }
            if ($Severity -and $parsedLevel -notmatch "[$Severity]") { continue }
            if ($FilterPackage -and ($line -notmatch $FilterPackage)) { continue }
            
            $results += [PSCustomObject]@{
                Date    = $matches['Date']
                Time    = $matches['Time']
                PID     = $parsedPid
                TID     = $matches['TID']
                Level   = $parsedLevel
                Tag     = $matches['Tag'].Trim()
                Message = $matches['Message']
                RawLine = $line
            }
        }
    }
    return $results
}
