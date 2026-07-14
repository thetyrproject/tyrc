@echo off
setlocal

echo.
echo ========================================
echo Cargo Check
echo ========================================
cargo check
if errorlevel 1 goto :failed

echo.
echo ========================================
echo Cargo Format
echo ========================================
cargo fmt --all
if errorlevel 1 goto :failed

echo.
echo ========================================
echo Cargo Clippy
echo ========================================
cargo clippy --workspace --all-targets --all-features -- -D warnings
if errorlevel 1 goto :failed

echo.
echo ========================================
echo Cargo Tests
echo ========================================
cargo test --workspace
if errorlevel 1 goto :failed

echo.
echo ========================================
echo Tyr Dance completed successfully!
echo ========================================
exit /b 0

:failed
echo.
echo Tyr Dance failed.
exit /b %ERRORLEVEL%