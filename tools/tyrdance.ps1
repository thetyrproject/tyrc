#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

function Run-Step {
    param(
        [string]$Name,
        [string]$Command
    )

    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "$Name" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Cyan

    Invoke-Expression $Command

    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "Tyr Dance failed during: $Name" -ForegroundColor Red
        exit $LASTEXITCODE
    }

    Write-Host "$Name passed." -ForegroundColor Green
}

Run-Step "Cargo Check" "cargo check"
Run-Step "Cargo Format" "cargo fmt --all"
Run-Step "Cargo Clippy" "cargo clippy --workspace --all-targets --all-features -- -D warnings"
Run-Step "Cargo Tests" "cargo test --workspace"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Tyr Dance completed successfully!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green