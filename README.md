# TON Vanity Search 🚀

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust CLI tool for brute-force searching TON (The Open Network) vanity addresses with parallel processing, customizable key ranges, and real-time progress reporting.

## English

### Description 📖

TON Vanity Search is a command-line utility written in Rust that allows you to find TON wallet addresses starting with a specific pattern (vanity addresses). It uses brute-force search within a specified range of private keys, leveraging parallel processing for optimal performance.

### Features ✨

- **Parallel Processing**: Utilizes Rayon for multi-threaded computation to speed up the search.
- **Customizable Ranges**: Specify start and end values for the key range to search.
- **Progress Reporting**: Displays real-time progress, including checked keys, elapsed time, and search speed.
- **TON v4r2 Support**: Generates addresses compatible with TON v4r2 wallets.
- **Efficient Key Generation**: Converts u64 values to 32-byte private keys for address derivation.

### Installation 🛠️

1. Ensure you have Rust installed (version 1.70 or later).
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ton-vanity-search.git
   cd ton-vanity-search
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Usage 💻

Run the tool with the following command:

```bash
cargo run -- --pattern <PATTERN> --start <START> --end <END> [--threads <THREADS>]
```

#### Arguments:
- `--pattern` or `-p`: The vanity pattern to search for (e.g., "EQabc").
- `--start` or `-s`: Start of the key range (u64).
- `--end` or `-e`: End of the key range (u64).
- `--threads` or `-t`: Number of threads to use (default: 4).

### Examples 📋

1. Search for an address starting with "EQtest" in the range 0 to 1,000,000 using 8 threads:
   ```bash
   cargo run -- --pattern "EQtest" --start 0 --end 1000000 --threads 8
   ```

2. Simple search with default threads:
   ```bash
   cargo run -- --pattern "EQvanity" --start 1000000 --end 2000000
   ```

### Dependencies 📦

| Dependency | Version | Description |
|------------|---------|-------------|
| clap | 4.0 | Command-line argument parser |
| ed25519-dalek | 2.0 | Ed25519 digital signatures |
| sha2 | 0.10 | SHA-2 hash functions |
| base64 | 0.21 | Base64 encoding/decoding |
| crc32fast | 1.3 | Fast CRC32 checksums |
| rayon | 1.7 | Parallel iterators |
| hex | 0.4 | Hexadecimal encoding |
| tonlib-core | 0.26.1 | TON blockchain library |

### Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Russian

### Описание 📖

TON Vanity Search - это высокопроизводительная утилита командной строки, написанная на Rust, которая позволяет находить адреса кошельков TON (The Open Network), начинающиеся с определенного шаблона (vanity-адреса). Она использует brute-force поиск в заданном диапазоне приватных ключей, используя параллельную обработку для оптимальной производительности.

### Особенности ✨

- **Параллельная обработка**: Использует Rayon для многопоточных вычислений для ускорения поиска.
- **Настраиваемые диапазоны**: Укажите начальное и конечное значения для диапазона ключей.
- **Отчет о прогрессе**: Отображает реальный прогресс, включая проверенные ключи, прошедшее время и скорость поиска.
- **Поддержка TON v4r2**: Генерирует адреса, совместимые с кошельками TON v4r2.
- **Эффективная генерация ключей**: Преобразует значения u64 в 32-байтовые приватные ключи для вывода адресов.

### Установка 🛠️

1. Убедитесь, что у вас установлен Rust (версия 1.70 или новее).
2. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/yourusername/ton-vanity-search.git
   cd ton-vanity-search
   ```
3. Соберите проект:
   ```bash
   cargo build --release
   ```

### Использование 💻

Запустите инструмент с помощью следующей команды:

```bash
cargo run -- --pattern <ШАБЛОН> --start <НАЧАЛО> --end <КОНЕЦ> [--threads <ПОТОКИ>]
```

#### Аргументы:
- `--pattern` или `-p`: Шаблон vanity для поиска (например, "EQabc").
- `--start` или `-s`: Начало диапазона ключей (u64).
- `--end` или `-e`: Конец диапазона ключей (u64).
- `--threads` или `-t`: Количество потоков (по умолчанию: 4).

### Примеры 📋

1. Поиск адреса, начинающегося с "EQtest" в диапазоне от 0 до 1,000,000 с использованием 8 потоков:
   ```bash
   cargo run -- --pattern "EQtest" --start 0 --end 1000000 --threads 8
   ```

2. Простой поиск с потоками по умолчанию:
   ```bash
   cargo run -- --pattern "EQvanity" --start 1000000 --end 2000000
   ```

### Зависимости 📦

| Зависимость | Версия | Описание |
|-------------|--------|----------|
| clap | 4.0 | Парсер аргументов командной строки |
| ed25519-dalek | 2.0 | Цифровые подписи Ed25519 |
| sha2 | 0.10 | Хэш-функции SHA-2 |
| base64 | 0.21 | Кодирование/декодирование Base64 |
| crc32fast | 1.3 | Быстрые контрольные суммы CRC32 |
| rayon | 1.7 | Параллельные итераторы |
| hex | 0.4 | Шестнадцатеричное кодирование |
| tonlib-core | 0.26.1 | Библиотека блокчейна TON |

### Вклад 🤝

Вклад приветствуется! Пожалуйста, не стесняйтесь отправлять Pull Request. Для крупных изменений, пожалуйста, сначала откройте issue, чтобы обсудить, что вы хотели бы изменить.

### Лицензия 📄

Этот проект лицензирован под лицензией MIT - подробности см. в файле [LICENSE](LICENSE).