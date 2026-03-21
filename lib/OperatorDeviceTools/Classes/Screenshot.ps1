class Screenshot {
    [string]$FilePath
    [string]$Label
    [datetime]$Timestamp
    [double]$SizeKb
    [string]$Hash
    [string]$DeviceSerial
    
    [string] ToString() {
        return "[$($this.Label)] $($this.FilePath) ($($this.SizeKb) KB) [MD5: $($this.Hash)]"
    }
}
