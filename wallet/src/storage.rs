use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub private_key: Vec<String>,
    pub public_key: Vec<String>,
}

pub struct Storage;

impl Storage {
    // Additional methods for testing file I/O with custom paths
    pub fn save_wallet(wallet_data: &WalletData, path: &str) {
        let data = serde_json::to_string_pretty(&wallet_data).unwrap();
        fs::write(path, data).unwrap();
    }

    pub fn load_wallet(path: &str) -> WalletData {
        let data = fs::read_to_string(path)
            .unwrap_or_else(|_| String::from("{ \"private_key\": [], \"public_key\": [] }"));
        serde_json::from_str(&data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_load_wallet() {
        // Create temporary file for testing
        let tmp_file = NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap().to_string();

        // Save some dummy data
        let wallet_data = WalletData {
            private_key: vec!["dummy_private_key".to_string()],
            public_key: vec!["dummy_public_key".to_string()],
        };

        // Save to temporary file
        Storage::save_wallet(&wallet_data, &tmp_path);
        
        // Load wallet data from the temporary file
        let loaded_wallet_data = Storage::load_wallet(&tmp_path);

        // Assert that the loaded data matches the saved data
        assert_eq!(wallet_data.private_key, loaded_wallet_data.private_key);
        assert_eq!(wallet_data.public_key, loaded_wallet_data.public_key);

        // Cleanup temp file
        fs::remove_file(tmp_path).unwrap();
    }
}
