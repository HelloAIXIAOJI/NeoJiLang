$version = (Get-Content -Path "Cargo.toml" | Select-String -Pattern 'version\s*=\s*"(.*?)"').Matches.Groups[1].Value
if (-not $version) {
    $version = "999.999.999"
}

$time = [System.TimeZoneInfo]::ConvertTimeBySystemTimeZoneId((Get-Date), "China Standard Time").ToString("yyyy-MM-dd-HH-mm-ss")

$backupDir = "update\$version-$time"
New-Item -ItemType Directory -Path $backupDir -Force

$platforms = @(
    @{target="x86_64-pc-windows-msvc"; name="win64"; ext="exe"},
    @{target="i686-pc-windows-msvc"; name="win32"; ext="exe"}
)

foreach ($platform in $platforms) {
    Write-Host "Now Building $($platform.name)..."
    
    cargo install cross 2>$null
    
        cargo build --release --target $platform.target
        $buildPath = "target\$($platform.target)\release\neo_jilang.$($platform.ext)"
    
    if (Test-Path $buildPath) {
        $destName = "NeoJiLang-$($platform.name).$($platform.ext)"
        $destPath = Join-Path $backupDir $destName
        Copy-Item $buildPath $destPath
        Write-Host "Build Done: $destPath" -ForegroundColor Green
    } else {
        Write-Host "BUILD ERROR: $($platform.name)" -ForegroundColor Red
    }
}

Write-Host "DONE!ON: $backupDir" -ForegroundColor Cyan