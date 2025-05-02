@echo off

:: Variables
set CLI_BIN_DIR=%USERPROFILES%\AppData\Local\Programs\ulvm\bin
set ULVM_BIN_DIR=%USERPROFILES%\.ulvm\bin

:: Create dirs
if not exist "%CLI_BIN_DIR%" mkdir "%CLI_BIN_DIR%"
if not exist "%ULVM_BIN_DIR%" mkdir "%ULVM_BIN_DIR%"

:: Copy
copy /Y ulvm.exe "%CLI_BIN_DIR%"
copy /Y ulvm_shim.exe "%ULVM_BIN_DIR%"

echo Installation complete ! 