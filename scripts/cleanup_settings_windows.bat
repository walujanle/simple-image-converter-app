@echo off
setlocal EnableDelayedExpansion

echo Simple Image Converter - Settings Cleanup
echo ==========================================
echo.
echo This will remove all saved settings including:
echo - Format preferences
echo - Quality settings
echo - Output folder preferences
echo - Theme preferences
echo.

set "APP_DATA=%APPDATA%\SimpleImageConverter"

if exist "%APP_DATA%" (
    echo Found settings at: %APP_DATA%
    echo.
    set /p "confirm=Are you sure you want to delete all settings? (Y/N): "
    if /i "!confirm!"=="Y" (
        rmdir /s /q "%APP_DATA%"
        echo.
        echo Settings cleaned successfully!
    ) else (
        echo.
        echo Operation cancelled.
    )
) else (
    echo.
    echo No settings found. Nothing to clean.
)

echo.
pause
