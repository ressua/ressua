use crate::key_management::KeyManager;
use crate::network::BitcoinNetwork;

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

    pub fn generate_new_address(&mut self) -> String {
        let (_private_key, public_key) = self.key_manager.generate_key_pair();
        let address = self.key_manager.public_key_to_address(&public_key);
        // Store the private key securely
        address
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64, reqwest::Error> {
        self.network.get_balance(address).await
    }
}
