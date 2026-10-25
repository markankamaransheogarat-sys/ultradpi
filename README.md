# ⚡ UltraDPI

> **Next-Gen Deep Packet Inspection (DPI / TSPU) Circumvention Engine.**  
> В 5 раз мощнее, быстрее и надежнее GoodbyeDPI. Работает на уровне ядра Windows без прокси и расширений.  
> **YouTube 4K 60fps без буферизации, Discord (войс-каналы UDP RTC + медиа), ChatGPT, Claude AI, Battlefield / EA Online, Steam, соцсети и трекеры.**

[![Release](https://img.shields.io/github/v/release/markankamaransheogarat-sys/ultradpi?color=brightgreen)](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg)](#быстрый-старт-в-1-клик)

---

## ⚡ Быстрый старт (в 1 клик, без компиляторов)

1. Скачайте готовый файл **[`ultradpi.exe`](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest/download/ultradpi.exe)** из раздела [Releases](https://github.com/markankamaransheogarat-sys/ultradpi/releases/latest).
2. Запустите `ultradpi.exe` двойным кликом (программа автоматически запросит права Администратора для загрузки сетевого драйвера ядра).
3. **Готово!** Никаких настроек в браузере, расширений и прокси не требуется. Всё работает сразу во всей системе.

---

## 🛠 Сборка из исходников (для разработчиков)

Если вы клонируете репозиторий:
```bash
git clone https://github.com/markankamaransheogarat-sys/ultradpi.git
cd ultradpi
cargo build --release
```
Готовый бинарник будет находиться по пути: `target/release/ultradpi.exe` (или сразу в корне репозитория).

---

## 🚀 Почему UltraDPI лучше GoodbyeDPI:

| Функция | GoodbyeDPI | ⚡ UltraDPI |
|---|---|---|
| **Установка** | Папка с 15 файлами и батниками | **1 самостоятельный .exe со всеми драйверами внутри** |
| **Discord Voice & RTC (UDP)** | ❌ Постоянные обрывы и RTC Connecting | **✅ Встроенный десинк голосовых UDP-портов 50000–65535** |
| **Игры (Battlefield, EA Online)** | ❌ Вылеты из матчей | **✅ Bypass игровых UDP/TCP портов (25200–25300 и др.)** |
| **Нейросети (ChatGPT, Claude AI)** | ⚠️ Ошибки подключения Cloudflare | **✅ Автоматическая TLS/SNI десинхронизация** |
| **YouTube 4K** | ⚠️ Частые подвисания | **✅ Разблокировка QUIC/HTTP3 + Fake Google ClientHello** |
| **Настройка браузера** | Требуется отключение Kyber/QUIC | **Ничего отключать не требуется — работает «из коробки»** |

---

## 👤 Автор
**markankamaransheogarat-sys**  
GitHub: [@markankamaransheogarat-sys](https://github.com/markankamaransheogarat-sys)

## 📄 Лицензия
MIT License.
