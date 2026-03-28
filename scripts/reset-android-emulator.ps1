# reset-android-emulator.ps1
# Kills running emulator processes and deletes all AVDs so they can be recreated fresh.

$avdmanager = "$env:LOCALAPPDATA\Android\Sdk\cmdline-tools\latest\bin\avdmanager.bat"
$emulatorBin = "$env:LOCALAPPDATA\Android\Sdk\emulator\emulator.exe"
$adb = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

# 1. Kill running emulators
Write-Host "Stopping running emulators..." -ForegroundColor Cyan
& $adb kill-server 2>$null
Get-Process -Name "emulator*", "qemu-system*" -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "  Killing $($_.Name) (pid $($_.Id))"
    Stop-Process -Id $_.Id -Force
}
Start-Sleep -Seconds 2

# 2. List AVDs
Write-Host "`nExisting AVDs:" -ForegroundColor Cyan
$avds = & $avdmanager list avd -c 2>$null
if (-not $avds) {
    Write-Host "  (none found)"
} else {
    $avds | ForEach-Object { Write-Host "  $_" }
}

# 3. Delete each AVD
$avds | Where-Object { $_ -match '\S' } | ForEach-Object {
    $name = $_.Trim()
    Write-Host "`nDeleting AVD: $name" -ForegroundColor Yellow
    & $avdmanager delete avd -n $name
}

Write-Host "`nDone. Run 'cargo tauri android dev' to create a fresh emulator." -ForegroundColor Green
