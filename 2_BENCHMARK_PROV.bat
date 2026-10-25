@echo off
chcp 65001 >nul
title UltraDPI - ISP Benchmark
cd /d "%~dp0"
target\release\ultradpi.exe benchmark
pause
