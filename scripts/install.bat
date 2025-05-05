@echo off
setlocal

:: === Display PATH instructions first ===
echo ====================================================
echo 🔧 ULVM CLI - Installation
echo ====================================================
echo.
echo To use ulvm from anywhere in your terminal:
echo ➤ Add this folder to your PATH:
echo   %%USERPROFILE%%\.ulvm\bin
echo.
echo 👉 If you're using PowerShell, you can add this to your profile:
echo   $env:PATH = "$env:USERPROFILE\.ulvm\bin;$env:PATH"
echo.
echo (Or manually update your environment variables)
echo ----------------------------------------------------
echo.

:: === Define target directory ===
set "ULVM_BIN_DIR=%USERPROFILE%\.ulvm\bin"

:: === Create directory if it doesn't exist ===
if not exist "%ULVM_BIN_DIR%" (
    echo 📁 Creating directory: %ULVM_BIN_DIR%
    mkdir "%ULVM_BIN_DIR%"
) else (
    echo 📁 Directory already exists: %ULVM_BIN_DIR%
)

:: === Copy executables ===
echo 📦 Copying ulvm.exe...
copy /Y ulvm.exe "%ULVM_BIN_DIR%" >nul

echo 📦 Copying ulvm_shim.exe...
copy /Y ulvm_shim.exe "%ULVM_BIN_DIR%" >nul

:: === Done ===
echo.
echo ✅ Installation completed successfully!
echo.

pause
endlocal
