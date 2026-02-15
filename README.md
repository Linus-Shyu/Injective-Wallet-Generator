# Injective Keygen

A **web app** (and optional desktop build) to generate [Injective](https://injective.com) wallets. Built with **Vue 3** and **Rust/WebAssembly** for the web; **Tauri 2** for desktop.

- **Generate** a new Injective address and private key (secp256k1 + Keccak256 + bech32 `inj`)
- **Copy** address or private key to clipboard
- **Export** wallet as a downloadable `.txt` file (web) or save to desktop (Tauri)

All key generation runs on your machine; nothing is sent to any server.

---

## Features

- One-click generation of Injective (inj) address and private key
- Copy address / private key with visual "Copied!" feedback
- Private key blurred until hover (reveal on hover)
- Web: download `injective_wallet.txt`; Desktop (Tauri): save to Desktop
- Injective-themed layout and colors (purple, typography, cards)

---

## Tech Stack

| Layer    | Stack |
|----------|--------|
| Frontend | Vue 3, TypeScript, Vite |
| Web      | Rust → WebAssembly (wasm-pack), injective_wallet pkg |
| Desktop  | Tauri 2 (optional); wallet logic in `src-tauri/src/lib.rs` |
| Crypto   | k256 (secp256k1), tiny_keccak, bech32, getrandom/rand |

---

## Prerequisites

- **Node.js** (v18+)
- **Rust** (stable, with `rustup`) — for building the Wasm crate and/or Tauri
- **wasm-pack** (optional): `cargo install wasm-pack` — used by `npm run build:wasm`

---

## Setup & Run

### Web (default)

```bash
git clone <your-repo-url>
cd rust-vue-demo

npm install
npm run dev
```

Build for production: `npm run build` (builds Wasm then Vite). Output in `dist/`. Preview: `npm run preview`.

### Desktop (Tauri)

```bash
npm run tauri dev
npm run tauri build
```

---

## Project Structure

```
rust-vue-demo/
├── src/                 # Vue frontend (App.vue uses Wasm for wallet)
├── wasm/                # Rust crate compiled to Wasm (injective_wallet)
│   ├── src/lib.rs       # generate_injective_wallet, get_wallet_export_text
│   └── pkg/             # wasm-pack output (used by Vue)
├── src-tauri/           # Tauri desktop app (wallet logic in lib.rs)
│   ├── src/lib.rs       # Wallet logic + Tauri commands
│   └── src/main.rs      # Entry point
├── package.json
└── README.md
```

---

## Disclaimer

For local wallet creation only. Secure your private keys; use at your own risk.
