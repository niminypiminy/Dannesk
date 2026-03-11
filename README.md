# Dannesk v0.3.0

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)](https://www.rust-lang.org/)
[![UI: Dioxus](https://img.shields.io/badge/UI-Dioxus-6f42c1)](https://dioxuslabs.com/)
[![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu-1f425f)](https://wgpu.rs/)

Dannesk is a native, non-custodial DeFi wallet for **Bitcoin** and the **XRP Ledger**. The app is built in **Rust** for performance, security, and reliability.

The app gives users complete control over their private keys while enabling powerful trading capabilities on **XRPL’s native CLOB**.

---

## ✨ Features

### Multi-Chain Wallet

Users can create a new wallet or import an existing one for:

- **Bitcoin (SegWit)**
- **XRP Ledger**

And make swaps on the **XRPL native order book (CLOB)**.

Supported assets include:

- **XRP**
- **RLUSD**
- **EUROP**
- **XSGD**
- **BTC**

Swaps occur **directly on-chain** with no centralized intermediary.

---

### BIP39 Passphrase Support

- Dannesk supports the optional BIP39 passphrase (sometimes called the 25th word).
- This allows for additional wallet security and the deterministic generation of multiple wallets from the same seed.
- This means that even if a bad actor acquires the 24-word key, they would still need the 25th word to derive the wallet.
- If the 25th word is stored only in one’s memory, the probability of an attacker obtaining both the 24 words and the passphrase is extremely low.  
- Furthermore, if the passphrase is sufficiently long (e.g., 15+ characters), brute-forcing the derived wallet becomes computationally infeasible. 

---

### Key Management

- Private keys are **encrypted locally using AES-256-GCM** upon wallet creation or import.
- Passphrase derivation uses **Argon2id**, an industry-standard password hashing algorithm.
- Users may remove encrypted keys at any time...reverting to cold storage. 
  
---

### Signing 
- Transactions are **signed locally on the user's device**.
- The signed transaction blob is then **broadcast to the network**.
- Sensitive memory is cleared using **zeroize** after signing operations.

---

## Installation

Download the latest release:

👉 https://dannesk.com

Supported platforms:

- Linux (.deb) 
- Windows (.exe) 

---

## License

Dannesk is licensed under the **GNU General Public License v3 (GPLv3)**.

See the `LICENSE` file for details.
