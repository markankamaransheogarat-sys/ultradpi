//! UltraDPI - Production-Grade Native Windows WinDivert System-Wide Kernel Bypass
//! Directly embeds WinDivert driver, cygwin and winws engine binaries into the single executable.
//! No proxies, no external configuration, no manual browser setup. 1-click execution.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use colored::*;

// Embedded binaries directly from Zapret / WinDivert core
const WINWS_EXE: &[u8] = include_bytes!("../bin/winws.exe");
const WINDIVERT_DLL: &[u8] = include_bytes!("../bin/WinDivert.dll");
const WINDIVERT64_SYS: &[u8] = include_bytes!("../bin/WinDivert64.sys");
const CYGWIN_DLL: &[u8] = include_bytes!("../bin/cygwin1.dll");

// Embedded fake packets
const FAKE_QUIC_GOOGLE: &[u8] = include_bytes!("../bin/quic_initial_www_google_com.bin");
const FAKE_QUIC_DBANK: &[u8] = include_bytes!("../bin/quic_initial_dbankcloud_ru.bin");
const FAKE_TLS_MAX: &[u8] = include_bytes!("../bin/tls_clienthello_max_ru.bin");

// Embedded hostlists
const LIST_GENERAL: &str = include_str!("../lists/list-general.txt");
const LIST_GOOGLE: &str = include_str!("../lists/list-google.txt");

fn is_elevated() -> bool {
    Command::new("net")
        .args(["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn relaunch_as_admin() {
    println!("{}", "[!] Запрос прав Администратора для загрузки драйвера перехвата пакетов WinDivert...".yellow());
    let current_exe = std::env::current_exe().expect("Failed to get current exe path");
    
    let status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!("Start-Process -FilePath '{}' -Verb RunAs", current_exe.to_string_lossy())
        ])
        .status();

    if let Ok(s) = status {
        if s.success() {
            std::process::exit(0);
        }
    }
}

fn unpack_runtime() -> PathBuf {
    let temp_dir = std::env::temp_dir().join("ultradpi_core");
    fs::create_dir_all(&temp_dir).expect("Failed to create temporary runtime directory");

    let write_file = |name: &str, bytes: &[u8]| {
        let p = temp_dir.join(name);
        if !p.exists() || fs::metadata(&p).map(|m| m.len()).unwrap_or(0) != bytes.len() as u64 {
            let mut f = fs::File::create(&p).expect(&format!("Failed to write {}", name));
            f.write_all(bytes).expect(&format!("Failed to write bytes to {}", name));
        }
    };

    write_file("winws.exe", WINWS_EXE);
    write_file("WinDivert.dll", WINDIVERT_DLL);
    write_file("WinDivert64.sys", WINDIVERT64_SYS);
    write_file("cygwin1.dll", CYGWIN_DLL);

    write_file("quic_initial_www_google_com.bin", FAKE_QUIC_GOOGLE);
    write_file("quic_initial_dbankcloud_ru.bin", FAKE_QUIC_DBANK);
    write_file("tls_clienthello_max_ru.bin", FAKE_TLS_MAX);

    let lists_dir = temp_dir.join("lists");
    fs::create_dir_all(&lists_dir).expect("Failed to create lists dir");

    let list_gen_path = lists_dir.join("list-general.txt");
    if !list_gen_path.exists() {
        fs::write(&list_gen_path, LIST_GENERAL).expect("Failed to write general list");
    }

    let list_goog_path = lists_dir.join("list-google.txt");
    if !list_goog_path.exists() {
        fs::write(&list_goog_path, LIST_GOOGLE).expect("Failed to write google list");
    }

    temp_dir
}

fn main() {
    println!("{}", "======================================================================".bright_cyan());
    println!("{}", "   ⚡ UltraDPI v1.0.0 | Native System-Wide TSPU / DPI Bypass Engine   ".bright_white().bold());
    println!("{}", "   Автоматический системный обход блокировок ТСПУ РКН без прокси      ".bright_green());
    println!("{}", "   Author: markankamaransheogarat-sys                                 ".bright_blue());
    println!("{}", "======================================================================".bright_cyan());
    println!();

    if !is_elevated() {
        relaunch_as_admin();
        return;
    }

    println!("{}", "[✓] Права Администратора подтверждены.".green());
    println!("{}", "[*] Распаковка системного драйвера WinDivert и движка в оперативную память...".cyan());
    
    let runtime_dir = unpack_runtime();
    let winws_path = runtime_dir.join("winws.exe");
    let lists_dir = runtime_dir.join("lists");

    let list_gen = lists_dir.join("list-general.txt");
    let list_goog = lists_dir.join("list-google.txt");
    let quic_google = runtime_dir.join("quic_initial_www_google_com.bin");
    let quic_dbank = runtime_dir.join("quic_initial_dbankcloud_ru.bin");
    let tls_fake_max = runtime_dir.join("tls_clienthello_max_ru.bin");

    println!("{}", "[*] Инициализация сетевого фильтра WinDivert для всех интерфейсов...".cyan());
    println!("{}", "[+] АКТИВНЫЕ ЦЕЛИ:".bright_white().bold());
    println!("{}", "    • YouTube: 4K без буферизации (QUIC + TLS десинхронизация)".bright_green());
    println!("{}", "    • Discord: Голосовые каналы (UDP RTC 50000-65535) + Медиашлюзы".bright_green());
    println!("{}", "    • Игры: Battlefield, EA Online, Steam (TCP/UDP bypass)".bright_green());
    println!("{}", "    • Нейросети & Сервисы: ChatGPT, Claude, Instagram, Rutracker".bright_green());
    println!();
    println!("{}", ">>> ОБХОД АКТИВЕН. Не закрывайте это окно во время работы в интернете <<<".bright_yellow().bold());
    println!("{}", "Нажмите Ctrl+C для остановки обхода.".white());
    println!();

    // Launch winws directly under the current console process
    let mut child: Child = Command::new(&winws_path)
        .current_dir(&runtime_dir)
        .args([
            "--wf-tcp=80,443,2053,2083,2087,2096,8443,25200-25300",
            "--wf-udp=443,19294-19344,50000-65535",
            "--filter-udp=443",
            &format!("--hostlist={}", list_gen.display()),
            "--dpi-desync=fake",
            "--dpi-desync-repeats=11",
            &format!("--dpi-desync-fake-quic={}", quic_google.display()),
            "--new",
            "--filter-udp=19294-19344,50000-65535",
            "--filter-l7=discord,stun",
            "--dpi-desync=fake",
            &format!("--dpi-desync-fake-discord={}", quic_dbank.display()),
            &format!("--dpi-desync-fake-stun={}", quic_dbank.display()),
            "--dpi-desync-repeats=6",
            "--new",
            "--filter-tcp=2053,2083,2087,2096,8443",
            "--hostlist-domains=discord.media",
            "--dpi-desync=fake,multidisorder",
            "--dpi-desync-split-pos=1,midsld",
            "--dpi-desync-repeats=11",
            "--dpi-desync-fooling=badseq",
            "--dpi-desync-fake-tls=0x00000000",
            "--dpi-desync-fake-tls=!",
            "--dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com",
            "--new",
            "--filter-tcp=443",
            &format!("--hostlist={}", list_goog.display()),
            "--ip-id=zero",
            "--dpi-desync=fake,multidisorder",
            "--dpi-desync-split-pos=1,midsld",
            "--dpi-desync-repeats=11",
            "--dpi-desync-fooling=badseq",
            "--dpi-desync-fake-tls=0x00000000",
            "--dpi-desync-fake-tls=!",
            "--dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com",
            "--new",
            "--filter-tcp=80,443",
            &format!("--hostlist={}", list_gen.display()),
            "--dpi-desync=fake,multidisorder",
            "--dpi-desync-split-pos=1,midsld",
            "--dpi-desync-repeats=11",
            "--dpi-desync-fooling=badseq",
            "--dpi-desync-fake-tls=0x00000000",
            "--dpi-desync-fake-tls=!",
            "--dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com",
            &format!("--dpi-desync-fake-http={}", tls_fake_max.display()),
            "--new",
            "--filter-tcp=80,443,8443,25200-25300",
            "--dpi-desync=fake,multidisorder",
            "--dpi-desync-any-protocol=1",
            "--dpi-desync-cutoff=n4",
            "--dpi-desync-split-pos=1,midsld",
            "--dpi-desync-repeats=11",
            "--dpi-desync-fooling=badseq",
            "--dpi-desync-fake-tls=0x00000000",
            "--dpi-desync-fake-tls=!",
            "--dpi-desync-fake-tls-mod=rnd,dupsid,sni=www.google.com",
            &format!("--dpi-desync-fake-http={}", tls_fake_max.display()),
            "--new",
            "--filter-udp=50000-65535,25200-25300",
            "--dpi-desync=fake",
            "--dpi-desync-repeats=10",
            "--dpi-desync-any-protocol=1",
            &format!("--dpi-desync-fake-unknown-udp={}", quic_dbank.display()),
            "--dpi-desync-cutoff=n2"
        ])
        .spawn()
        .expect("Failed to start WinDivert bypass core process");

    let _ = child.wait();
}
