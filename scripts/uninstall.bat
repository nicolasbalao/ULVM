@echo off
setlocal

:: === Define target directory ===
set "ULVM_BIN_DIR=%USERPROFILE%\.ulvm\bin"


echo 📁 Removing dir : %ULVM_BIN_DIR%
rmdir "%ULVM_BIN_DIR%"

:: === Done ===
echo.
echo ✅ Uninstallation complete!
echo.

pause
endlocal
