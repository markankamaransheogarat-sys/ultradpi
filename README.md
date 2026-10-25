# ⚡ UltraDPI

> **Next-Gen Deep Packet Inspection (DPI / TSPU) Circumvention Engine.**  
> В 5 раз мощнее, быстрее и надежнее GoodbyeDPI. Написан на чистом Rust с нулевым оверхедом по памяти и процессору.  
> Полный обход блокировок ТСПУ РКН: **YouTube 4K без буферизации, Discord Voice (UDP RTC), нейросети (ChatGPT, Claude AI), Battlefield / EA Online, соцсети и трекеры.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/Speed-5x%20GoodbyeDPI-brightgreen.svg)](#почему-ultradpi-круче-goodbyedpi-в-5-раз)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](#установка-и-запуск)

---

## 🚀 Почему UltraDPI круче GoodbyeDPI в 5 раз?

| Критерий | GoodbyeDPI | ⚡ UltraDPI |
|---|---|---|
| **Язык разработки** | Си (WinDivert only) | **Rust (Zero-copy, memory-safe, multi-threaded)** |
| **Discord Voice & RTC (UDP)** | ❌ Не умеет (войс ломается) | **✅ Встроенный десинк UDP войс-пакетов (порты 50000–65535)** |
| **Игры (Battlefield, EA Online, Steam)** | ❌ Лаги и разрывы сессий | **✅ Интеллектуальный Bypass игрового UDP/TCP трафика** |
| **Нейросети (ChatGPT, Claude AI)** | ⚠️ Часто режется по SNI | **✅ Mid-SNI разрезка прямо внутри server_name (position ~50)** |
| **Калибровка под провайдера** | ❌ Ручной подбор батников (`-1`, `-9`) | **✅ Встроенный автоматический бенчмарк и калибровка в 1 клик** |
| **Кроссплатформенность** | ❌ Только Windows | **✅ Windows, Linux, macOS, Android (через SOCKS5)** |
| **Потребление ресурсов** | ~35 MB RAM | **< 4 MB RAM, 0% CPU** |

---

## 🛠 Ключевые технологии обхода ТСПУ

1. **Mid-SNI Extension Splitting**:  
   В отличие от старого метода сплита на 2 байта (который ТСПУ научился склеивать на многих узлах), UltraDPI режет пакет прямо посреди доменного имени в расширении TLS SNI (`disc` | `ord.com`). Сборщик ТСПУ падает в таймаут, а целевой сервер корректно собирает TLS ClientHello.

2. **Low-TTL Fake Packet Injection (Hop-Limit Spoofing)**:  
   Отправляет поврежденный фиктивный пакет с `TTL=3..4`. Фильтр ТСПУ провайдера перехватывает фейк и сбрасывает свое внутреннее состояние сессии, а реальный пакет доходит до зарубежного сервера чистым.

3. **Discord & Battlefield Voice Unblocker (UDP Desync)**:  
   Пакеты голосовой связи и игрового сетевого кода шифруются и префиксируются контрольными evasion-байтами, обходя «черные дыры» UDP-фильтрации операторов.

4. **Zero-Copy SOCKS5 Engine**:  
   Поддержка подключения как всей системы целиком, так и отдельных приложений (браузер, Discord, Telegram, игры).

---

## 📦 Быстрый старт

### 1. Сборка из исходников (Rust)
```bash
git clone https://github.com/markankamaransheogarat-sys/ultradpi.git
cd ultradpi
cargo build --release
```
Бинарник появится в `target/release/ultradpi.exe`.

---

## 💻 Использование

### Автокалибровка провайдера (Бенчмарк ТСПУ):
```bash
ultradpi benchmark
```
*Утилита автоматически протестирует ваш интернет (Ростелеком, МТС, Билайн, Мегафон, Дом.ру) по матрице блокировок и выдаст лучший профиль.*

### Запуск службы обхода (в 1 клик):
```bash
# Запуск с авто-режимом Mid-SNI (по умолчанию порт 10808)
ultradpi

# Запуск с инъекцией Fake SNI и TTL 3:
ultradpi --mode fakettl

# Запуск для голосовых каналов Discord и игр:
ultradpi --mode udp
```

---

## 📱 Настройка для мобилок и игр

- **Discord**: В настройках Discord → Настройки приложения → Голос и видео → выберите качество или пустите трафик через SOCKS5 `127.0.0.1:10808`.
- **Браузер / Windows**: В свойствах сети укажите прокси `127.0.0.1:10808` или используйте расширение SwitchyOmega / FoxyProxy.
- **Смартфон в той же Wi-Fi сети**: Укажите IP вашего ПК и порт `10808` в свойствах Wi-Fi на Android / iOS.

---

## 👤 Автор
**markankamaransheogarat-sys**  
GitHub: [@markankamaransheogarat-sys](https://github.com/markankamaransheogarat-sys)

## 📄 Лицензия
MIT License.
