:: === Define target directory ===
set "ULVM_BIN_DIR=%USERPROFILE%\.ulvm\bin"

echo ====================================================
echo 🗑️  ULVM CLI - Uninstallation
echo ====================================================
echo.

:: === Check if the directory exists ===
if exist "%ULVM_BIN_DIR%" (
    echo 📁 Removing directory: %ULVM_BIN_DIR%
    rmdir /S /Q "%ULVM_BIN_DIR%"
    echo ✅ Uninstallation complete!
) else (
    echo ⚠️  Directory not found: %ULVM_BIN_DIR%
    echo Nothing to uninstall.
)

echo.
pause
endlocal
