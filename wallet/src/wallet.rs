use crate::key_management::KeyManager;
use crate::network::BitcoinNetwork;

pub struct Wallet {
    key_manager: KeyManager,
    network: BitcoinNetwork,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            key_manager: KeyManager::new(),
            network: BitcoinNetwork::new(),
        }
    }

    pub fn generate_new_address(&mut self) -> String {
        let (private_key, public_key) = self.key_manager.generate_key_pair();
        let address = self.key_manager.public_key_to_address(&public_key);
        // Store the private key securely
        address
    }

    pub async fn get_balance(&self, address: &str) -> f64 {
        self.network.get_balance(address).await
    }
}
