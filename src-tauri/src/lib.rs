// Wallet generation logic (shared core); Tauri entry in run().
use bech32::{ToBase32, Variant};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;
use rand::rngs::OsRng;
use tiny_keccak::{Hasher, Keccak};

/// Generate a new Injective wallet (address + private key).
pub fn generate_injective_wallet_impl() -> Result<serde_json::Value, String> {
    let secret_key = SecretKey::random(&mut OsRng);
    let private_key_hex = format!("0x{}", hex::encode(secret_key.to_bytes()));

    let public_key_bytes = secret_key.public_key().to_encoded_point(false);
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&public_key_bytes.as_bytes()[1..]);
    hasher.finalize(&mut hash);

    let inj_address = bech32::encode("inj", (&hash[12..]).to_base32(), Variant::Bech32)
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "private_key": private_key_hex,
        "address": inj_address
    }))
}

/// Return wallet export file content (for desktop save or web download).
pub fn get_wallet_export_text(address: &str, private_key: &str) -> String {
    format!(
        "Injective Wallet Export\nGenerated at: {}\n\nAddress: {}\nPrivate Key: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        address,
        private_key
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate_injective_wallet,
            save_wallet_to_desktop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_injective_wallet() -> Result<serde_json::Value, String> {
    generate_injective_wallet_impl()
}

#[tauri::command]
fn save_wallet_to_desktop(address: String, private_key: String) -> Result<String, String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Failed to get home directory")?;
    let file_path = format!("{}/Desktop/injective_wallet.txt", home_dir);
    let content = get_wallet_export_text(&address, &private_key);
    std::fs::write(&file_path, content).map_err(|e| e.to_string())?;
    Ok("Wallet saved to desktop successfully".to_string())
}
