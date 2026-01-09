@echo off
setlocal
title Simple Image Converter - Auto Build Script (Windows)

echo ===================================================
echo     Simple Image Converter - Release Builder
echo ===================================================
echo.

:: 1. Verify Prerequisites
echo [1/4] Verifying Prerequisites...

:: Check Cargo
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 goto cargo_missing
echo   - Rust toolchain found.

:: Check VCPKG_ROOT
if "%VCPKG_ROOT%"=="" goto vcpkg_missing
echo   - VCPKG_ROOT found: %VCPKG_ROOT%

:: Check Vcpkg Dependencies (heif)
if not exist "%VCPKG_ROOT%\installed\x64-windows\bin\heif.dll" goto heif_missing
echo   - Dependencies check passed.

echo.
echo [2/4] Building Application (Release Mode)...
echo.

:: 2. Build App
call cargo build --release
if %ERRORLEVEL% NEQ 0 goto build_fail

echo.
echo [3/4] Gathering Dependencies and Packaging...

set "OUTPUT_DIR=build_release_windows"
if exist "%OUTPUT_DIR%" (
    echo   - Cleaning previous build artifacts...
    rmdir /s /q "%OUTPUT_DIR%"
)
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

:: Copy Executable
echo   - Copying executable...
copy /Y "target\release\simple-image-converter-app.exe" "%OUTPUT_DIR%\" >nul
if %ERRORLEVEL% NEQ 0 goto copy_fail

:: Copy VCPKG DLLs
echo   - Copying Application Dependencies (heif, libde265, libx265)...
copy /Y "%VCPKG_ROOT%\installed\x64-windows\bin\heif.dll" "%OUTPUT_DIR%\" >nul
copy /Y "%VCPKG_ROOT%\installed\x64-windows\bin\libde265.dll" "%OUTPUT_DIR%\" >nul
copy /Y "%VCPKG_ROOT%\installed\x64-windows\bin\libx265.dll" "%OUTPUT_DIR%\" >nul

:: Copy VCRuntime DLLs (App-Local Deployment)
echo   - Copying Visual C++ Runtime (App-Local)...
set "SYS32=%SystemRoot%\System32"
copy /Y "%SYS32%\vcruntime140.dll" "%OUTPUT_DIR%\" >nul
copy /Y "%SYS32%\vcruntime140_1.dll" "%OUTPUT_DIR%\" >nul
copy /Y "%SYS32%\msvcp140.dll" "%OUTPUT_DIR%\" >nul

echo.
echo.
echo [4/4] Finalizing...

:: Copy Utility Scripts
echo   - Copying cleanup script...
copy /Y "scripts\cleanup_settings_windows.bat" "%OUTPUT_DIR%\" >nul

echo.
echo ===================================================
echo               BUILD SUCCESSFUL!
echo ===================================================
echo.
echo Build artifacts placed in: %CD%\%OUTPUT_DIR%
echo.
pause
exit /b 0

:: =========================================================
:: ERROR HANDLERS
:: =========================================================

:cargo_missing
echo [ERROR] Rust toolchain (cargo) not found!
echo Please install Rust from https://rustup.rs/
pause
exit /b 1

:vcpkg_missing
echo [ERROR] VCPKG_ROOT environment variable is not set.
echo.
echo This script needs VCPKG_ROOT to locate required DLLs.
echo Please set it to your vcpkg installation directory.
echo Example: set VCPKG_ROOT=C:\path\to\vcpkg
pause
exit /b 1

:heif_missing
echo [ERROR] heif.dll not found in vcpkg bin folder!
echo Checked path: "%VCPKG_ROOT%\installed\x64-windows\bin\heif.dll"
echo Please install it: vcpkg install libheif:x64-windows
pause
exit /b 1

:build_fail
echo.
echo [ERROR] Build failed! Check the output above.
pause
exit /b 1

:copy_fail
echo [ERROR] Failed to copy executable.
pause
exit /b 1
