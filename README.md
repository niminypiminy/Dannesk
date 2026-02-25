# Dannesk v0.2.0

![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![UI: Dioxus](https://img.shields.io/badge/UI-Dioxus-6f42c1)
![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu-1f425f)

Dannesk is a native, non-custodial decentralized finance (DeFi) application that enables secure management and trading of XRP and Bitcoin assets. Written entirely in Rust, it is designed from the ground up for high performance and uncompromising client-side security.

🛡️ Overview
Dannesk operates on a strict local-first, client-side security model. All keys, transactions, and cryptographic operations are handled exclusively on your device. There are no centralized custody servers, and your sensitive data is never transmitted.

✨ Features
Multi-Chain Support: Native wallet management for Bitcoin (Native SegWit) and the XRP Ledger (XRPL). Users can import or generate new wallets for either chain.

XRPL Native DEX (CLOB): Trade directly on the XRPL Central Limit Order Book for a fraction of a cent. Currently supporting stablecoins like RLUSD (Ripple USD) and EUROP (Schuman Financial).

Advanced Key Generation: Generates secure 24-word BIP39 seed phrases with support for an optional 25th-word passphrase, allowing for hidden wallet accounts and plausible deniability.

Cold Storage Ready: Keep keys securely encrypted on your device, or easily purge them for a cold storage approach.

Fully Client-Side: All transaction signing happens entirely on your local machine.

🏗️ Architecture & Tech Stack
Dannesk leverages the modern Rust ecosystem to deliver a memory-safe, non-custodial wallet with native performance. The core logic is completely decoupled from the UI.

1. High-Performance Frontend
Dioxus + Blitz: The UI is built using Dioxus and rendered via the wgpu + vello stack (Vulkan, Metal), compiling directly to machine code for minimal CPU and RAM usage.

Reactive State: UI state is managed via Dioxus signals and Watch channels. A centralized coroutine (context.rs) listens for updates, ensuring real-time balance and order book synchronization without blocking the main thread.

2. Asynchronous Core & Networking
Concurrent Backend: Powered by tokio for highly concurrent, non-blocking operations.

Custom Proxy: Utilizes a custom-built proxy written in Rust using axum and tower to securely route necessary requests.

Low Latency: Communicates directly with XRPL and Bitcoin nodes via WebSockets for instantaneous market data and transaction broadcasting.

3. Hardened Security Layer
Encrypted at Rest: Wallet data is encrypted using AES-256-GCM. We use Argon2id for key derivation, strictly adhering to OWASP recommended parameters (64MB RAM, 3 iterations, 4 lanes) to maximize resistance against GPU-based brute-force attacks.

Aggressive Memory Zeroing: Leveraging Rust's ownership model and the zeroize crate, all sensitive data (passphrases, seed guards, vectors, and strings) is aggressively wiped from the stack and memory immediately upon drop. Your keys are never logged or leaked.

🚀 Roadmap
🚧 In Progress
Bitcoin as Collateral: Integrating native BTC-backed collateralization for DeFi lending and borrowing workflows.

CLOB Expansion: Adding support for additional stablecoin tokens on the XRPL DEX.

📦 Installation
Download the latest release for your platform (Linux and Windows supported):

👉 dannesk.com

📄 License
Dannesk is licensed under the GNU General Public License v3 (GPLv3). See the LICENSE file for details.
