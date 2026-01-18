# Dannesk

![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![UI: Dioxus](https://img.shields.io/badge/UI-Dioxus-6f42c1)
![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu-1f425f)

**Dannesk** is a native, non-custodial decentralized finance (DeFi) application that enables secure management and trading of **XRP** and **Bitcoin** assets — fully client-side and written entirely in **Rust**.

---

## Overview

Dannesk is built around a local-first security model.  
All keys, transactions, and cryptographic operations are handled directly on the user’s device. No backend custody, remote signing, or centralized control is involved.

The application runs as a fully native desktop app with GPU-accelerated rendering.

---

## Features

- Create and import wallets for **Bitcoin** and **XRP Ledger (XRPL)**
- Trade stablecoins directly on the **XRPL decentralized exchange (DEX)**
  - Supported assets:
    - **RLUSD** (Ripple)
    - **EUROP** (Schuman Financial)
- Fully **client-side transaction signing**
- **Non-custodial** key management
- AES-256 encrypted key storage with passphrase protection
- Cold storage–friendly wallet architecture
- Built entirely in **Rust**

---

## Architecture

- 100% client-side application
- Native frontend powered by **Dioxus Blitz**
  - Experimental **wgpu + vello** rendering stack
- Cryptography and transaction logic implemented in Rust
- Designed for security, transparency, and native performance

---

## Installation

Download and install Dannesk from:

👉 **https://dannesk.com**

---

## Quick Start

1. Launch Dannesk.
2. Create or import a wallet (Bitcoin or XRPL).
3. Manage balances or trade supported assets on the XRPL DEX.
4. All operations and signing occur locally on your device.

---

## License

Dannesk is licensed under the **GNU General Public License v3 (GPLv3)**.  
See the [LICENSE](LICENSE) file for details.

