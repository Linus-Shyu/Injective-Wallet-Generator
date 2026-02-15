---

```markdown
# Injective Keygen

A local desktop app to generate [Injective](https://injective.com) wallets. Built with **Tauri 2** (Rust) and **Vue 3**, with an Injective-inspired UI.

- **Generate** a new Injective address and private key (secp256k1 + Keccak256 + bech32 `inj`)
- **Copy** address or private key to clipboard
- **Export** wallet to a timestamped `.txt` file on your desktop

All key generation runs on your machine; nothing is sent to any server.

---

## Features

- One-click generation of Injective (inj) address and private key
- Copy address / private key with visual "Copied!" feedback
- Private key blurred until hover (reveal on hover)
- Save wallet to desktop as `injective_wallet.txt` (with timestamp)
- Injective-themed layout and colors (purple, typography, cards)

---

## Tech Stack

| Layer    | Stack |
|----------|--------|
| Frontend | Vue 3, TypeScript, Vite |
| Backend  | Rust (Tauri 2) |
| Crypto   | k256 (secp256k1), tiny_keccak, bech32, rand |

---

## Prerequisites

- **Node.js** (v18+)
- **Rust** (stable, with `rustup`)
- **macOS**: Xcode Command Line Tools (or full Xcode)
- **Windows**: Microsoft C++ Build Tools, WebView2
- **Linux**: `webkit2gtk`, etc. (see [Tauri docs](https://tauri.app/start/prerequisites))

---

## Setup & Run

```bash
git clone <your-repo-url>
cd rust-vue-demo

npm install
npm run tauri dev
```

Build: `npm run tauri build` — output under `src-tauri/target/release/`.

---

## Project Structure

```
rust-vue-demo/
├── src/                 # Vue frontend (App.vue, injective-theme.css)
├── src-tauri/           # Rust (main.rs: generate_injective_wallet, save_wallet_to_desktop)
├── package.json
└── README.md
```

---

## Disclaimer

For local wallet creation only. Secure your private keys; use at your own risk.
```

