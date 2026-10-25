@echo off
chcp 65001 >nul
title UltraDPI Launcher
cd /d "%~dp0"

echo =======================================================
echo    ⚡ UltraDPI v1.0.0 - Запуск обхода блокировок ТСПУ
echo    YouTube 4K ^| Discord Voice ^| AI Tools ^| Games
echo =======================================================
echo.

if not exist "target\release\ultradpi.exe" (
    echo [*] Компиляция релизной версии UltraDPI (первый запуск)...
    cargo build --release
)

echo.
echo [1] Запустить обход (Режим по умолчанию: Mid-SNI)
echo [2] Запустить калибровку под провайдера (Бенчмарк ТСПУ)
echo [3] Запустить для Discord Voice и Игр (UDP Evasion)
echo [4] Запустить с Fake-пакетами (TTL=3)
echo.
set /p choice="Выберите режим (1-4) [Enter = 1]: "

if "%choice%"=="2" (
    target\release\ultradpi.exe benchmark
    pause
    exit /b
)
if "%choice%"=="3" (
    target\release\ultradpi.exe --mode udp
    pause
    exit /b
)
if "%choice%"=="4" (
    target\release\ultradpi.exe --mode fakettl
    pause
    exit /b
)

target\release\ultradpi.exe
pause
