use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub private_keys: Vec<String>,
}

pub struct Storage;

impl Storage {
    pub fn load_wallet() -> WalletData {
        let data = fs::read_to_string("wallet_data.json").unwrap_or_else(|_| String::from("{ \"private_keys\": [] }"));
        serde_json::from_str(&data).unwrap()
    }

    pub fn save_wallet(wallet_data: &WalletData) {
        let data = serde_json::to_string_pretty(&wallet_data).unwrap();
        fs::write("wallet_data.json", data).unwrap();
    }
}
