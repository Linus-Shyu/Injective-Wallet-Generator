<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Wallet {
  private_key: string;
  address: string;
}

const wallet = ref<Wallet | null>(null);
const loading = ref(false);
const message = ref("Ready to secure the future.");
const copiedField = ref<"address" | "private_key" | null>(null);

async function copyToClipboard(text: string): Promise<void>{
  try {
    await navigator.clipboard.writeText(text);
    copiedField.value = text === wallet.value?.address ? "address" : "private_key";
    message.value = "Copied to clipboard.";
    setTimeout(() => {
      copiedField.value = null;
    }, 2000);
  } catch {
    message.value = "Copy failed.";
  }

}
async function generateWallet() {
  loading.value = true;
  message.value = "Executing cryptographic derivation...";

  try {
    const result = await invoke<Wallet>("generate_injective_wallet");
    wallet.value = result;
    message.value = "Wallet successfully initialized.";
  } catch (err) {
    message.value = "Error: " + err;
  } finally {
    loading.value = false;
  }
}

async function saveToFile() {
  if (!wallet.value) return;

  try {
    const response = await invoke<string>("save_wallet_to_desktop", {
      address: wallet.value.address,
      privateKey: wallet.value.private_key,
    });
    message.value = response;
  } catch (err) {
    message.value = "Export failed: " + err;
  }
}
</script>

<template>
  <div class="app-root">
    <header class="app-header">
      <div class="container header-inner">
        <div class="logo">
          <img src="/injective-logo.png" alt="Injective" class="logo-mark" width="32" height="32" />
          <span class="logo-text">INJECTIVE <span class="logo-accent">// KEYGEN</span></span>
        </div>
        <span class="eyebrow-tag text-eyebrow">LOCAL_WALLET</span>
      </div>
    </header>

    <main class="app-main">
      <section class="hero">
        <div class="hero-bg" aria-hidden="true"></div>
        <div class="container hero-content">
          <h1 class="text-h2 hero-title">The future of finance starts here.</h1>
          <p class="hero-subtitle text-body-5">
            Institutional-grade wallet generation on your local machine.
          </p>
        </div>
      </section>

      <section class="content-section">
        <div class="container content-inner">
          <div class="action-zone">
            <button
              type="button"
              class="btn btn-primary inj-btn-generate"
              :disabled="loading"
              @click="generateWallet"
            >
              <span v-if="!loading">Generate random wallet</span>
              <span v-else class="loading-text">Computing…</span>
            </button>

            <Transition name="fade-slide">
              <div v-if="wallet" class="card card-elevated result-card">
                <span class="text-eyebrow result-tag">SECURE_MODULE_V1</span>

                <div class="data-row">
                  <span class="data-label text-label-whyte-2">Injective address</span>
                  <div class="data-value-wrap">
                    <span class="data-value mono">{{ wallet.address }}</span>
                    <button type="button" class="btn-copy" title="Copy address" @click="copyToClipboard(wallet.address)">
                      {{ copiedField === "address" ? "Copied!" : "Copy" }}
                    </button>
                  </div>
    
                </div>

                <div class="data-row">
                  <span class="data-label text-label-whyte-2">Private key (hover to reveal)</span>
                  <div>
                    <button type="button" class="btn-copy" title="Copy private key" @click="copyToClipboard(wallet.private_key)">
                    {{copiedField === "private_key" ? "Copied!" : "Copy" }}
                    </button>
                  </div>
                  <div class="data-value mono secret">{{ wallet.private_key }}</div>
                </div>

                <button type="button" class="btn btn-secondary btn-full" @click="saveToFile">
                  Save to desktop (.txt)
                </button>
              </div>
            </Transition>
          </div>
        </div>
      </section>
    </main>

    <footer class="app-footer">
      <div class="container footer-inner">
        <div class="status">
          <span class="status-dot" :class="{ 'is-loading': loading }" aria-hidden="true"></span>
          <span class="text-label-whyte-2">{{ message }}</span>
        </div>
        <div class="legal text-label-whyte-2">Injective Foundation © 2026</div>
      </div>
    </footer>
  </div>
</template>

<style scoped>
.app-root {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--Primary-Ocean-Main);
}

/* Header — fixed, over hero */
.app-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding: 1rem 0;
  border-bottom: 1px solid rgba(212, 224, 255, 0.12);
  background: transparent;
  transition: background 0.3s;
}

.header-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-mark {
  width: 28px;
  height: 28px;
  flex-shrink: 0;
  object-fit: contain;
  color: var(--Web-INJ-White);
}

.logo-text {
  font-size: 0.95rem;
  font-weight: 400;
  letter-spacing: 0.02em;
  color: var(--Web-INJ-White);
}

.logo-accent {
  color: var(--Shade-Sky-400);
}

.eyebrow-tag {
  color: var(--Tertiary-Lime-Main);
  border: 1px solid var(--Tertiary-Lime-Main);
  padding: 4px 8px;
  border-radius: 4px;
}

/* Main */
.app-main {
  flex: 1;
  padding-top: 0;
}

/* Hero — Injective-style gradient + copy */
.hero {
  position: relative;
  min-height: 50vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 6rem 0 4rem;
}

.hero-bg {
  position: absolute;
  inset: 0;
  background: radial-gradient(ellipse 80% 60% at 50% 0%, var(--Primary-Ocean-Tint) 0%, var(--Primary-Ocean-Main) 60%);
  pointer-events: none;
}

.hero-content {
  position: relative;
  z-index: 1;
  text-align: center;
}

.hero-title {
  color: var(--Web-INJ-White);
  margin: 0 0 1rem;
  max-width: 16ch;
  margin-left: auto;
  margin-right: auto;
}

.hero-subtitle {
  color: var(--Shade-Sky-400);
  margin: 0;
  opacity: 0.95;
}

/* Content section */
.content-section {
  padding: 2rem 0 4rem;
  position: relative;
  z-index: 2;
}

.content-inner {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
}

.action-zone {
  width: 100%;
  max-width: 480px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;
}

.inj-btn-generate {
  min-width: 260px;
  background-color: var(--Primary-Ocean-Tint) !important;
  color: var(--Web-INJ-White) !important;
  filter: brightness(1.05);
}

.inj-btn-generate:hover:not(:disabled) {
  background-color: var(--Primary-Ocean-Tint) !important;
  filter: brightness(1.05);
}

.result-card .btn-secondary {
  background: rgba(77, 61, 255, 0.12);
  border-color: var(--Primary-Ocean-Main);
  color: var(--Primary-Ocean-Main);
}

.result-card .btn-secondary:hover:not(:disabled) {
  background: rgba(77, 61, 255, 0.2);
  border-color: var(--Primary-Ocean-Main);
}

.result-card {
  width: 100%;
  padding: 2rem;
}

.result-tag {
  display: block;
  color: var(--Shade-Grey-600);
  margin-bottom: 1.5rem;
}

.data-row {
  margin-bottom: 1.25rem;
}

.data-label {
  display: block;
  color: var(--Shade-Grey-600);
  margin-bottom: 0.5rem;
}

.data-value {
  font-size: 14px;
  line-height: 1.5;
  word-break: break-all;
  color: var(--Secondary-Midnight-900);
}

.mono {
  font-family: ui-monospace, "SF Mono", "Cascadia Code", monospace;
}

.secret {
  filter: blur(8px);
  transition: filter 0.25s ease;
  cursor: pointer;
}

.secret:hover {
  filter: none;
}

.data-value-wrap {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.data-value-wrap .data-value {
  flex: 1;
  min-width: 0;
}

.btn-copy {
  flex-shrink: 0;
  min-width: 5.25rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 14px;
  font-size: 13px;
  font-weight: 500;
  letter-spacing: 0.02em;
  border-radius: var(--radius-s);
  border: 1px solid var(--Shade-Sky-300);
  background: var(--Shade-Sky-50);
  color: var(--Secondary-Midnight-900);
  cursor: pointer;
  transition: border-color 0.2s, background 0.2s, color 0.2s, transform 0.15s;
}

.btn-copy:hover {
  border-color: var(--Primary-Ocean-Main);
  background: var(--Primary-Snow-Main);
  color: var(--Primary-Ocean-Main);
}

.btn-copy:active {
  transform: scale(0.98);
}

/* 复制成功时的状态 */
.btn-copy.copied {
  border-color: var(--Tertiary-Lime-Main);
  background: rgba(206, 255, 200, 0.35);
  color: var(--Secondary-Midnight-900);
}

.btn-full {
  width: 100%;
  margin-top: 0.5rem;
}

.loading-text {
  opacity: 0.9;
}

/* Footer — 与主背景同色紫 */
.app-footer {
  margin-top: auto;
  padding: 1.5rem 0;
  background: var(--Primary-Ocean-Main);
  color: var(--Web-INJ-White);
  border-top: 1px solid rgba(255, 255, 255, 0.15);
}

.footer-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 1rem;
}

.app-footer .text-label-whyte-2 {
  color: var(--Shade-Sky-200);
}

.status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--Tertiary-Lime-Main);
  flex-shrink: 0;
}

.status-dot.is-loading {
  animation: pulse 1.5s ease-in-out infinite;
}

.legal {
  color: var(--Shade-Sky-400);
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.2); }
}

/* Transitions */
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.4s ease, transform 0.4s ease;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(16px);
}
</style>
