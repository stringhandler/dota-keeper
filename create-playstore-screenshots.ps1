#Requires -Version 5.1
<#
.SYNOPSIS
    Generates Play Store screenshots for all required device form factors.

.DESCRIPTION
    1. Reads SCREENSHOT_STEAM_ID from .env.screenshots
    2. Fetches real match data from OpenDota API
    3. Captures screenshots via Playwright for phone, 7" tablet, 10" tablet, Chromebook
    4. Outputs PNGs to meta\screenshots\<device>\

.EXAMPLE
    .\create-playstore-screenshots.ps1
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = $PSScriptRoot

# ---------------------------------------------------------------------------
# 1. Load .env.screenshots
# ---------------------------------------------------------------------------

$EnvFile = Join-Path $Root ".env.screenshots"
if (-not (Test-Path $EnvFile)) {
    Write-Error @"
.env.screenshots not found.
Copy the example and fill in your Steam64 ID:

    Copy-Item .env.screenshots.example .env.screenshots

Then edit .env.screenshots and set:
    SCREENSHOT_STEAM_ID=76561198XXXXXXXXX
"@
    exit 1
}

foreach ($line in Get-Content $EnvFile) {
    $line = $line.Trim()
    if (-not $line -or $line.StartsWith("#")) { continue }
    $parts = $line -split "=", 2
    if ($parts.Count -eq 2) {
        $key = $parts[0].Trim()
        $val = $parts[1].Trim().Trim('"').Trim("'")
        [System.Environment]::SetEnvironmentVariable($key, $val, "Process")
    }
}

if (-not $env:SCREENSHOT_STEAM_ID) {
    Write-Error "SCREENSHOT_STEAM_ID is not set in .env.screenshots"
    exit 1
}

Write-Host ""
Write-Host "=== Dota Keeper - Play Store Screenshots ===" -ForegroundColor Cyan
Write-Host "Steam ID : $($env:SCREENSHOT_STEAM_ID.Substring(0, 10))..." -ForegroundColor DarkGray
Write-Host ""

# ---------------------------------------------------------------------------
# 2. Fetch mock data from OpenDota
# ---------------------------------------------------------------------------

Write-Host "Step 1/2  Fetching match data from OpenDota..." -ForegroundColor Yellow
npm run screenshots:fetch
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to fetch mock data. Check your Steam ID and that your OpenDota profile is public."
    exit 1
}

Write-Host ""

# ---------------------------------------------------------------------------
# 3. Capture screenshots
# ---------------------------------------------------------------------------

Write-Host "Step 2/2  Capturing screenshots..." -ForegroundColor Yellow
npm run screenshots
if ($LASTEXITCODE -ne 0) {
    Write-Error "Screenshot capture failed."
    exit 1
}

# ---------------------------------------------------------------------------
# 4. Summary
# ---------------------------------------------------------------------------

$OutDir = Join-Path $Root "meta\screenshots"
$Files = @(Get-ChildItem -Path $OutDir -Recurse -Filter "*.png" -ErrorAction SilentlyContinue)

Write-Host ""
Write-Host "Done!" -ForegroundColor Green
Write-Host "$($Files.Count) screenshots saved to: $OutDir" -ForegroundColor Green
Write-Host ""

# Open the folder in Explorer
Start-Process explorer.exe $OutDir
