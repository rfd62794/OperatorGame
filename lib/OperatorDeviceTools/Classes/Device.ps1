class Device {
    [string]$Serial
    [string]$State
    [string]$Model
    [int]$ApiLevel
    [bool]$DebugEnabled
    [double]$StorageFreeGb
    [string]$AndroidVersion
    
    [void] Refresh() {
        $this.State = (Invoke-AdbCommand -Serial $this.Serial -Command "get-state").Trim()
        
        $apiStr = Invoke-AdbCommand -Serial $this.Serial -Command "shell getprop ro.build.version.sdk"
        if ([int]::TryParse($apiStr.Trim(), [ref]$this.ApiLevel)) {}
        
        $this.AndroidVersion = (Invoke-AdbCommand -Serial $this.Serial -Command "shell getprop ro.build.version.release").Trim()
        $this.Model = (Invoke-AdbCommand -Serial $this.Serial -Command "shell getprop ro.product.model").Trim()
        
        $dfStr = Invoke-AdbCommand -Serial $this.Serial -Command "shell df /data | grep /data" -NoErrorCheck
        if ($dfStr -match '\s+(\d+)\s+\d+%\s+/data') {
            # Usually df prints in 1K-blocks, convert to GB
            $this.StorageFreeGb = [math]::Round([double]$matches[1] / 1024 / 1024, 2)
        } else {
            Write-Warning "Could not parse storage info from df output for $($this.Serial)"
            $this.StorageFreeGb = 0
        }
        $this.DebugEnabled = $true
    }
    
    [bool] IsHealthy() {
        return ($this.State -eq "device") -and $this.DebugEnabled
    }
    
    [string] ToString() {
        $gbStr = if ($this.StorageFreeGb) { "$($this.StorageFreeGb) GB free" } else { "Unknown storage" }
        return "$($this.Serial) - $($this.Model) (API $($this.ApiLevel), $gbStr)"
    }
}
