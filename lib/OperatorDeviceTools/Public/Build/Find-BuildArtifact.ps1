function Find-BuildArtifact {
    param(
        [ValidateSet("ApkRelease", "ApkDebug", "Aab", "Jks")]
        [string]$ArtifactType,
        [string]$SearchPath = $null
    )
    
    <#
    .SYNOPSIS
    Discover build artifacts in repo or build directory.
    #>
    
    if (-not $SearchPath) {
        $SearchPath = $Script:OperatorDeviceTools.RepositoryRoot
    }
    
    $filter = switch ($ArtifactType) {
        "ApkRelease" { "*release*.apk" }
        "ApkDebug" { "*debug*.apk" }
        "Aab" { "*.aab" }
        "Jks" { "*.jks" }
        default { "*.*" }
    }
    
    $files = Get-ChildItem -Path $SearchPath -Filter $filter -Recurse -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending
    
    if ($files) {
        return $files[0].FullName
    }
    return $null
}
