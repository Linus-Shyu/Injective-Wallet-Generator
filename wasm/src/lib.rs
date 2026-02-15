//! Injective wallet generation — WebAssembly build for web deployment.
//! Wallet logic: secp256k1 key, Keccak256, bech32 inj address.

use bech32::{ToBase32, Variant};
use getrandom::getrandom;
use js_sys::Date;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;
use tiny_keccak::{Hasher, Keccak};
use wasm_bindgen::prelude::*;

/// Generate a new Injective wallet (address + private key).
/// Returns a JSON string: `{"address":"inj...","private_key":"0x..."}`.
#[wasm_bindgen(js_name = "generate_injective_wallet")]
pub fn generate_injective_wallet() -> Result<String, JsValue> {
    // 1. Random 32 bytes for secret key (browser: getrandom uses crypto.getRandomValues)
    let mut bytes = [0u8; 32];
    getrandom(&mut bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let secret_key =
        SecretKey::from_slice(&bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let private_key_hex = format!("0x{}", hex::encode(secret_key.to_bytes()));

    // 2. Public key -> Keccak256 (last 20 bytes = Ethereum-style address)
    let public_key_bytes = secret_key.public_key().to_encoded_point(false);
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&public_key_bytes.as_bytes()[1..]);
    hasher.finalize(&mut hash);

    // 3. Bech32 encode for Injective (inj)
    let inj_address = bech32::encode("inj", (&hash[12..]).to_base32(), Variant::Bech32)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let out = serde_json::json!({
        "address": inj_address,
        "private_key": private_key_hex
    });
    Ok(out.to_string())
}

/// Return wallet export file content (for browser download).
#[wasm_bindgen(js_name = "get_wallet_export_text")]
pub fn get_wallet_export_text(address: &str, private_key: &str) -> String {
    let now = Date::new_0();
    let timestamp = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        now.get_full_year(),
        now.get_month() + 1,
        now.get_date(),
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    );
    format!(
        "Injective Wallet Export\nGenerated at: {}\n\nAddress: {}\nPrivate Key: {}\n",
        timestamp, address, private_key
    )
}

// Optional: re-export init for wasm-pack generated bootstrap (no-op if not needed)
// wasm-pack generates an init() that loads the .wasm; we don't need extra init here.
