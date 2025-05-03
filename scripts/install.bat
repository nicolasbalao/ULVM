@echo off
setlocal

:: === Display PATH instructions first ===
echo ====================================================
echo 🔧 ULVM CLI - Installation
echo ====================================================
echo.
echo Pour utiliser ulvm depuis n'importe où dans votre terminal :
echo ➤ Ajoutez ce dossier à votre PATH :
echo   %%USERPROFILE%%\.ulvm\bin
echo.
echo 👉 Si vous utilisez PowerShell, vous pouvez ajouter ceci à votre profil :
echo   $env:PATH = "$env:USERPROFILE\.ulvm\bin;$env:PATH"  I
echo.
echo (Ou modifiez manuellement les variables d’environnement)
echo ----------------------------------------------------
echo.

:: === Define target directory ===
set "ULVM_BIN_DIR=%USERPROFILE%\.ulvm\bin"

:: === Create directory if it doesn't exist ===
if not exist "%ULVM_BIN_DIR%" (
    echo 📁 Création du dossier : %ULVM_BIN_DIR%
    mkdir "%ULVM_BIN_DIR%"
) else (
    echo 📁 Dossier déjà présent : %ULVM_BIN_DIR%
)

:: === Copy executables ===
echo 📦 Copie de ulvm.exe...
copy /Y ulvm.exe "%ULVM_BIN_DIR%" >nul

echo 📦 Copie de ulvm_shim.exe...
copy /Y ulvm_shim.exe "%ULVM_BIN_DIR%" >nul

:: === Done ===
echo.
echo ✅ Installation terminée avec succès !
echo.

pause
endlocal
