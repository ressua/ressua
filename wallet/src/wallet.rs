use crate::key_management::KeyManager;
use crate::network::BitcoinNetwork;
use crate::storage::{Storage, WalletData};

pub struct Wallet {
    key_manager: KeyManager,
    network: BitcoinNetwork,
}

impl Wallet {
    pub fn new(is_testnet: bool) -> Self {
        Wallet {
            key_manager: KeyManager::new(is_testnet),
            network: BitcoinNetwork::new(is_testnet),
        }
    }

    pub fn generate_new_address(&mut self, &public_key) -> String {
        // let (_private_key, public_key) = self.key_manager.generate_key_pair();
        let address = self.key_manager.public_key_to_address(&public_key);
        // Store the private key securely
        address
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64, reqwest::Error> {
        self.network.get_balance(address).await
    }

    pub fn generate_and_save_key_pair(&self) {
        let (private_key, public_key) = self.key_manager.generate_key_pair();
   
        let mut wallet_data = WalletData {
            private_key: vec!["dummy_private_key".to_string()],
            public_key: vec!["dummy_public_key".to_string()],
        };
        wallet_data.private_key.push(private_key.to_string());
        wallet_data.public_key.push(public_key.to_string());
        Storage::save_wallet(&wallet_data, "wallet.json");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_management::KeyManager;
    use crate::network::BitcoinNetwork;
    use crate::storage::WalletData;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_generate_new_address() {
        let mut wallet = Wallet::new(true); // Testnet wallet
        let address = wallet.generate_new_address(public_key);
        assert!(address.starts_with('m') || address.starts_with('n') || address.starts_with('2'), 
                "Address should start with m, n or 2 for testnet");
    }

    #[tokio::test]
    async fn test_get_balance() {
        let mut wallet = Wallet::new(true); // Testnet wallet
        let address = wallet.generate_new_address(public_key);

        // Use a mock balance for testing, assuming the network layer is implemented
        // and returns a balance (or adjust depending on the actual implementation)
        match wallet.get_balance(&address).await {
            Ok(balance) => assert!(balance >= 0.0, "Balance should be non-negative"),
            Err(e) => panic!("Failed to get balance: {}", e),
        }
    }

    #[tokio::test]
    async fn test_generate_and_save_key_pair() {
        // Create a temporary file for testing
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // Override the storage file path for testing
        std::env::set_var("WALLET_DATA_PATH", temp_path);

        // Create a test wallet
        let wallet = Wallet::new(true); // Testnet wallet

        // Generate and save a new key pair
        wallet.generate_and_save_key_pair();

        // Load the wallet data from the temporary file
        let wallet_data = Storage::load_wallet(temp_path);

        // Check that the key pair was saved correctly
        assert_eq!(wallet_data.private_key.len(), 1);
        assert_eq!(wallet_data.public_key.len(), 1);

        // Clean up the environment variable
        std::env::remove_var("WALLET_DATA_PATH");
    }
}
