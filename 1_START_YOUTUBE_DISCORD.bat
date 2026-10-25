@echo off
chcp 65001 >nul
:: Проверка прав администратора
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo =======================================================
    echo  [!] ВНИМАНИЕ: Для перехвата пакетов Windows WinDivert
    echo      требуются права Администратора!
    echo =======================================================
    echo.
    echo Перезапускаю от имени Администратора...
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

title ⚡ UltraDPI - Полный обход ТСПУ (Системный уровень)
cd /d "%~dp0"

echo ======================================================================
echo   ⚡ UltraDPI v1.0.0 - SYSTEM-WIDE WINWS ^& WINDIVERT ENGINE
echo   Обход без прокси в браузере: Discord (Voice UDP + Media), YouTube 4K, 
echo   ChatGPT, Claude, Battlefield, Steam, Instagram, Rutracker
echo ======================================================================
echo.

set "BIN=%~dp0bin\"
set "LISTS=%~dp0lists\"
cd /d %BIN%

echo [*] Запуск драйвера WinDivert и инъектора Fake-TLS/Disorder...
echo [*] Все приложения в Windows теперь работают напрямую без прокси!
echo.
echo [!] Не закрывайте это окно, пока пользуетесь интернетом.
echo.

"%BIN%winws.exe" --wf-tcp=80,443,2053,2083,2087,2096,8443,25200-25300 --wf-udp=443,19294-19344,50000-65535 ^
--filter-udp=443 --hostlist="%LISTS%list-general.txt" --dpi-desync=fake --dpi-desync-repeats=11 --dpi-desync-fake-quic="%BIN%quic_initial_www_google_com.bin" --new ^
--filter-udp=19294-19344,50000-65535 --filter-l7=discord,stun --dpi-desync=fake --dpi-desync-fake-discord="%BIN%quic_initial_dbankcloud_ru.bin" --dpi-desync-fake-stun="%BIN%quic_initial_dbankcloud_ru.bin" --dpi-desync-repeats=6 --new ^
--filter-tcp=2053,2083,2087,2096,8443 --hostlist-domains=discord.media --dpi-desync=fake,multidisorder --dpi-desync-split-pos=1,midsld --dpi-desync-repeats=11 --dpi-desync-fooling=badseq --dpi-desync-fake-tls=0x00000000 --dpi-desync-fake-tls=^! --dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com --new ^
--filter-tcp=443 --hostlist="%LISTS%list-google.txt" --ip-id=zero --dpi-desync=fake,multidisorder --dpi-desync-split-pos=1,midsld --dpi-desync-repeats=11 --dpi-desync-fooling=badseq --dpi-desync-fake-tls=0x00000000 --dpi-desync-fake-tls=^! --dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com --new ^
--filter-tcp=80,443 --hostlist="%LISTS%list-general.txt" --dpi-desync=fake,multidisorder --dpi-desync-split-pos=1,midsld --dpi-desync-repeats=11 --dpi-desync-fooling=badseq --dpi-desync-fake-tls=0x00000000 --dpi-desync-fake-tls=^! --dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com --dpi-desync-fake-http="%BIN%tls_clienthello_max_ru.bin" --new ^
--filter-tcp=80,443,8443,25200-25300 --dpi-desync=fake,multidisorder --dpi-desync-any-protocol=1 --dpi-desync-cutoff=n4 --dpi-desync-split-pos=1,midsld --dpi-desync-repeats=11 --dpi-desync-fooling=badseq --dpi-desync-fake-tls=0x00000000 --dpi-desync-fake-tls=^! --dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com --dpi-desync-fake-http="%BIN%tls_clienthello_max_ru.bin" --new ^
--filter-udp=50000-65535,25200-25300 --dpi-desync=fake --dpi-desync-repeats=10 --dpi-desync-any-protocol=1 --dpi-desync-fake-unknown-udp="%BIN%quic_initial_dbankcloud_ru.bin" --dpi-desync-cutoff=n2

pause
