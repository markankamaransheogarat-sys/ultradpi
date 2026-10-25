@echo off
chcp 65001 >nul
title UltraDPI - YouTube 4K ^& Discord Voice
cd /d "%~dp0"
target\release\ultradpi.exe --mode midsni
pause
