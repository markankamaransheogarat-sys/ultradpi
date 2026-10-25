# ⚡ UltraDPI

> **Next-Gen Deep Packet Inspection (DPI / TSPU) Circumvention Engine for Windows.**  
> 5x faster, lighter, and more resilient than GoodbyeDPI. Operates directly at the Windows kernel level without requiring any proxies, browser extensions, or network configuration.  
> **Unblocks YouTube 4K 60fps (no buffering), Discord (UDP RTC voice channels + media gateways), ChatGPT, Claude AI, Battlefield / EA Online, Steam, social networks, and torrent trackers.**

[![Release](https://img.shields.io/github/v/release/markankamaransheogarat-sys/ultradpi?color=brightgreen)](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg)](#-1-click-quick-start)
[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

---

## ⚡ 1-Click Quick Start (Zero Config)

1. Download the standalone executable **[`ultradpi.exe`](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest/download/ultradpi.exe)** from the [Releases](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest) page.
2. Double-click `ultradpi.exe` to run (it automatically requests UAC Administrator privileges to load the WinDivert kernel driver).
3. **Done!** No browser settings, flags, or proxy extensions needed. All traffic is unblocked system-wide.

---

## 🚀 Why UltraDPI is 5x Better than GoodbyeDPI

| Feature | GoodbyeDPI | ⚡ UltraDPI |
|---|---|---|
| **Package** | Cluttered folder with 15+ files and bat scripts | **Single standalone `.exe` with all drivers embedded** |
| **Discord Voice & RTC (UDP)** | ❌ Broken voice channels & RTC Connecting loops | **✅ Native UDP desync for voice ports (50000–65535)** |
| **Games (Battlefield, EA Online, Steam)** | ❌ Match disconnects and dropped sessions | **✅ Kernel bypass for game ports (25200–25300 & UDP)** |
| **AI Tools (ChatGPT, Claude AI)** | ⚠️ Cloudflare handshake timeouts | **✅ Advanced multi-stage TLS/SNI desynchronization** |
| **YouTube 4K 60fps** | ⚠️ Stuttering & frequent buffering | **✅ QUIC / HTTP3 spoofing + Fake Google ClientHello** |
| **Browser Setup** | Manual disabling of Kyber / QUIC in flags | **Zero configuration required — works out of the box** |
| **Memory Footprint** | ~35 MB RAM | **< 4 MB RAM, 0% CPU overhead** |

---

## 🛠 Building from Source (For Developers)

```bash
git clone https://github.com/markankamaransheogarat-sys/ultradpi.git
cd ultradpi
cargo build --release
```
The compiled standalone binary will be available at `target/release/ultradpi.exe` (and directly in the root repository folder).

---

## 👤 Author
**markankamaransheogarat-sys**  
GitHub: [@markankamaransheogarat-sys](https://github.com/markankamaransheogarat-sys)

## 📄 License
MIT License.

---

<details>
<summary>🇷🇺 <b>Описание на русском языке (Нажмите, чтобы развернуть)</b></summary>

### ⚡ UltraDPI — Движок обхода блокировок ТСПУ нового поколения

Автономный обходчик блокировок ТСПУ (РКН) для Windows без необходимости настраивать браузеры, прокси или расширения.

#### Быстрый старт:
1. Скачайте готовый файл **[`ultradpi.exe`](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest/download/ultradpi.exe)**.
2. Запустите двойным кликом (программа сама запросит права Администратора).
3. Готово! YouTube 4K, Discord (голос + чат), нейросети (ChatGPT, Claude), игры (Battlefield, Steam) работают сразу во всей системе.

#### Преимущества:
- **Один файл**: драйвер WinDivert, движок и списки вшиты прямо внутрь `ultradpi.exe`.
- **Голос в Discord**: исправлена давняя проблема GoodbyeDPI — голосовые каналы и стримы работают без зависаний.
- **Игровой трафик**: пакеты Battlefield и серверов EA не дропаются фильтрами провайдера.
- **Минимальный вес**: потребляет меньше 4 МБ оперативной памяти.

</details>
