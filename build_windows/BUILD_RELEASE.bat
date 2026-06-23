@echo off
title COLDCASE - Build Release
color 0B
cd /d "%~dp0\.."

echo ============================================================
echo COLDCASE - BUILD RELEASE
echo xtr4ng3
echo ============================================================
echo.

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo No se encontro Cargo/Rust.
    echo Instala Rust desde rustup.rs
    pause
    exit /b
)

cargo build --release

if %errorlevel% neq 0 (
    echo Fallo compilacion.
    pause
    exit /b
)

rmdir /s /q CLIENTE_PORTABLE 2>nul
mkdir CLIENTE_PORTABLE
copy /Y target\release\coldcase.exe CLIENTE_PORTABLE\coldcase.exe
copy /Y README.md CLIENTE_PORTABLE\README.txt
xcopy /E /I /Y dashboard CLIENTE_PORTABLE\dashboard
xcopy /E /I /Y rules CLIENTE_PORTABLE\rules

echo.
echo Build listo:
echo CLIENTE_PORTABLE\coldcase.exe
pause
