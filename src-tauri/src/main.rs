// What libraries are we using?
use tauri::command;
use bech32::{self, ToBase32, Variant};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;
use rand::rngs::OsRng;
use tiny_keccak::{Hasher, Keccak};
use std::fs::File;
use std::io::Write;

// generate Injective wallet 
#[command]
fn generate_injective_wallet() -> Result<serde_json::Value, String> {
    // 1. Generate a random private key
    let secret_key = SecretKey::random(&mut OsRng);
    let private_key_hex = format!("0x{}", hex::encode(secret_key.to_bytes()));

    // 2. Output the public key and calculate the ether hash
    let public_key_bytes = secret_key.public_key().to_encoded_point(false);
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&public_key_bytes.as_bytes()[1..]);
    hasher.finalize(&mut hash);

    // 3. exchange injective address
    // 修正点：在 hash[12..] 前加上 &，将其转为引用类型
    let inj_address = bech32::encode("inj", (&hash[12..]).to_base32(), Variant::Bech32)
        .map_err(|e| e.to_string())?;

    // 4.Result back to vue
    Ok(serde_json::json!({
        "private_key": private_key_hex,
        "address": inj_address
    }))
}

// Save the address to a file
#[command]
fn save_wallet_to_desktop(address: String,private_key: String) -> Result<String, String> {
    // 1. Visit oprating system's home directory
    let home_dir = std::env::var("HOME").map_err(|_| "Failed to get home directory")?;

    // 2.Find the directory of injective wallet
    let file_path = format!("{}/Desktop/injective_wallet.txt", home_dir);

    // 3. prepare the content of the file
    let content = format!(
        "Injective Wallet Export\nGenerated at: {}\n\nAddress: {}\nPrivate Key: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        address,
        private_key
    );

    // 4. Create the file and write the content
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;

    // 5. Exchange text change to text flow
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

    // 6. Result back to vue
    Ok("Wallet saved to desktop successfully".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate_injective_wallet,
            save_wallet_to_desktop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}