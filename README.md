# VPS Meter v0.4.0

[![Repository](https://img.shields.io/badge/Repository-GitHub-181717?style=for-the-badge&logo=github)](https://github.com/mqteooo317/vps-meter)
[![Author](https://img.shields.io/badge/Author-mateo-000000?style=for-the-badge&logo=github)](https://github.com/mqteooo317)
[![Discord](https://img.shields.io/badge/Discord-mateo-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/users/1279870617197482055)

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square)](LICENSE)

**VPS performance benchmarking tool.**

---

## Description

VPS Meter is a command-line benchmarking tool written in Rust that measures CPU, RAM, disk, and network performance of any Linux server. Designed for hosting providers, system administrators, and developers who need fast, reliable, and comparable performance metrics.

**Key features:**
- Parallel benchmark execution (CPU, RAM, disk, network run simultaneously)
- Performance scores (0-100) per category and overall
- Comparison mode to measure improvements
- JSON output for automation and integration
- Runs in 2-4 seconds on modern hardware
- No dependencies - single static binary

---

## Installation

### From source

```bash
git clone https://github.com/mqteooo317/vps-meter.git
cd vps-meter
cargo build --release
sudo cp target/release/vps-meter /usr/local/bin/
```

## Pre-built binaries

- Download from GitHub Releases

```bash
# Linux x86_64
wget https://github.com/mqteooo317/vps-meter/releases/download/v0.4.0/vps-meter-x86_64-unknown-linux-gnu.tar.gz
tar -xzf vps-meter-x86_64-unknown-linux-gnu.tar.gz
sudo mv vps-meter /usr/local/bin/
```

---

## Usage

## Basic benchmark

```bash
vps-meter
```

## JSON output (for scripting)

```bash
vps-meter --json
```

## Save baseline

```bash
vps-meter --json > baseline.json
```

## Compare with baseline

```bash
vps-meter --compare baseline.json
```

## Quiet mode (no ANSI colors)

```bash
vps-meter --quiet --json
```

## Verbose output

```bash
vps-meter --verbose
```

## Command line options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Show additional metrics (RAM available, disk used, etc.) |
| `-j, --json` | Output results in JSON format |
| `-q, --quiet` | Suppress ANSI colors and banner |
| `-c, --compare <FILE>` | Compare results with a baseline JSON file |

---

## Contributors

-  @mqteooo317 - Lead Developer

---

## License

- Apache License 2.0.
