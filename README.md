# Dannesk v0.2.0

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)](https://www.rust-lang.org/)
[![UI: Dioxus](https://img.shields.io/badge/UI-Dioxus-6f42c1)](https://dioxuslabs.com/)
[![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu-1f425f)](https://wgpu.rs/)

Dannesk is a native, non-custodial DeFi wallet for Bitcoin and XRP. Built with Rust for high-performance & security. 

## ✨ Features

- **Multi-Chain:** Native support for Bitcoin (SegWit) and XRPL.
- **DEX Trading:** Trade RLUSD, EUROP, and more on the native XRPL CLOB.
- **Local Security:** BIP39 seeds with 25th-word passphrase support. All keys and signing stay on-device.
- **Privacy:** Local data encrypted via AES-256-GCM and Argon2id. Memory is zeroed on drop.

## 🛠 Tech Stack

- **Core:** Rust, Tokio, Axum.
- **UI:** Dioxus + Blitz (GPU-rendered via wgpu).
- **Security:** Argon2id & `zeroize`.

## 🛤 Roadmap

- [ ] **BTC Collateral:** Native Bitcoin-backed DeFi workflows.
- [ ] **Adding More Tokens:** Prioritizing stablecoins.

## Installation

Download the latest release for your platform (Linux and Windows supported):

👉 https://dannesk.com

## License

Dannesk is licensed under the **GNU General Public License v3 (GPLv3)**.  
See the [LICENSE](./LICENSE) file for details.
