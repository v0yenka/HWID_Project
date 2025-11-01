# Rust HWID Generator

This project is a Hardware ID (HWID) generator written in Rust. It combines different pieces of system information (like memory, CPU cores, hostname, OS version, and disk info) to
create a unique fingerprint for your device. The generated HWID is then encrypted using AES-256 Base64 for privacy and security.

---

## Features

- Generate a unique hardware ID based on your system.
- Choose which components to include using a clean bitmask enum.
- Encrypt the HWID using AES-256 Base64.
- Written entirely in Rust — my first project exploring system programming and Rust features.

## Supports:

- Total system memory
- Hostname
- OS version
- Kernel version
- CPU core count
- Disk names

Note: MAC addresses are omitted for now due to crate/API limitations, but may be added in future updates.

---

## Usage

### 1. Clone the repository:
```bash
git clone https://github.com/your-username/hwid_generator.git
cd hwid_generator
```
---

### 2. Build and run:
```bash
cargo run
```
### 3. You can modify which components are included in src/main.rs by adjusting the Combinations bitmask:
```bash
let comb = Combinations::TotalMemBytes | Combinations::HostName | Combinations::CoresCount;
```
### 4. The program will output your encrypted HWID:
```bash
Generated HWID: <encrypted_string>
```
---

## Dependencies:
- bitmask-enum
- magic-crypt
- sysinfo
---

## Author

v0yenka
