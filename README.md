<div align="center">
    <h1>Oxi-Fetch 🌳</h1>
    <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=for-the-badge" alt="License" />
    <br>
    <img src="https://img.shields.io/crates/d/oxi-fetch?style=for-the-badge&logo=rust&label=Crates.io%20Downloads&color=orange&labelColor=202020" alt="Crates Downloads" />
</div>

<p align="center">
  <b>Language / Язык:</b>
  <a href="#english">English</a> • <a href="#russian">Русский</a>
</p>

---

<a id="english"></a>
## English README.md

**Oxi-Fetch** is a fast cross-platform system information tool written in Rust, displaying OS logos and hardware stats

---

## 🚀 Features

* 🖥 **Cross-Platform Support:** Native support for both Linux and Windows environments
* 🎨 **ASCII Logos with Color:** Automatically detects your operating system and displays a vibrant, color-coded ASCII logo
* 💾 **Detailed Hardware Info:** Fetches CPU model, real-time CPU usage, GPU detection (via lspci on Linux and WMI on Windows), and NVIDIA GPU utilization if available
* 📊 **System Metrics:** Displays RAM usage, disk usage, active network local IP, OS kernel version, uptime, and desktop environment (DE/WM) or shell details
* ⚡️ **Performance:** Built efficiently in Rust using the powerful sysinfo library for minimal resource overhead

---

## 📥 Installation

**Oxi-Fetch** is published on [crates.io](https://crates.io/crates/oxi-fetch). If you have Rust and Cargo installed, you can install the tool with a single command:

```bash
cargo install oxi-fetch
```

---

## 📜 License

Distributed under the **Apache-2.0** License.


---

<a id="russian"></a>
## Русский README.md

**Oxi-Fetch** — это быстрый кроссплатформенный инструмент для получения системной информации, написанный на Rust, который отображает логотипы операционных систем и статистику по оборудованию

---

## 🚀 Особенности

* 🖥 **Кроссплатформенность:** Нативная поддержка операционных систем Linux и Windows
* 🎨 **Цветные ASCII-логотипы:** Автоматическое определение ОС с выводом детализированного и цветного ASCII-логотипа
* 💾 **Подробная информация об железе:** Определение модели процессора, мониторинг загрузки CPU в реальном времени, определение видеокарты (через lspci на Linux и WMI на Windows), а также отслеживание нагрузки NVIDIA GPU (nvidia-smi)
* 📊 **Системные метрики:** Отовод использования RAM, дискового пространства, локального IP-адреса сети, версии ядра ОС, времени работы (uptime), а также используемой графической оболочки (DE/WM) или командной оболочки (Shell)
* ⚡️ **Производительность:** Написано на Rust с использованием эффективной библиотеки sysinfo с минимальным потреблением ресурсов

---

## 📥 Установка

### Через Cargo (Рекомендуемый способ ⭐)

Утилита опубликована на [crates.io](https://crates.io/crates/oxi-fetch). Если у вас установлен Rust и менеджер пакетов Cargo, вы можете установить `oxi-fetch` всего одной командой:

```bash
cargo install oxi-fetch
```

---

## 📜 Лицензия

Проект распространяется под лицензией **Apache-2.0**
