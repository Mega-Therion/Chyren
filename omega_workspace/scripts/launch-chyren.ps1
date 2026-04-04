$usbPaths = @("D:", "E:", "F:", "G:")
$omegaRoot = $null

foreach ($drive in $usbPaths) {
    if (Test-Path "$drive\CHYREN") {
        $omegaRoot = "$drive\CHYREN"
        break
    }
}

if (-not $omegaRoot) {
    $omegaRoot = $env:OMEGA_ROOT -or (Get-Location).Path
}

$env:OMEGA_ROOT = $omegaRoot
$env:OMEGA_HOST_CACHE = $env:OMEGA_HOST_CACHE -or "$env:USERPROFILE\.omega-host-cache\chyren"

New-Item -ItemType Directory -Force -Path $env:OMEGA_HOST_CACHE | Out-Null

Write-Host "OmegA Workspace: $omegaRoot"
Write-Host "Host Cache: $env:OMEGA_HOST_CACHE"
Write-Host ""

Push-Location "$omegaRoot\workspace\OmegA-Next"
cargo run --package omega-cli -- @args
Pop-Location
