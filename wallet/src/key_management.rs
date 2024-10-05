use bitcoin::key::{PrivateKey, PublicKey};
use bitcoin::network::Network;
use bitcoin::NetworkKind;
use secp256k1::{rand, Secp256k1, SecretKey};

pub struct KeyManager {
    secp: Secp256k1<secp256k1::All>,
    network: Network,      // Store the network (Bitcoin or Testnet)
    network_kind: NetworkKind, // Store the network kind (Main or Test)
}

impl KeyManager {
    pub fn new(is_testnet: bool) -> Self {
        // Set the appropriate network and network kind based on `is_testnet`
        let (network, network_kind) = if is_testnet {
            (Network::Testnet, NetworkKind::Test)
        } else {
            (Network::Bitcoin, NetworkKind::Main)
        };
        
        KeyManager {
            secp: Secp256k1::new(),
            network,
            network_kind,
        }
    }

    pub fn generate_key_pair(&self) -> (PrivateKey, PublicKey) {
        // Generate a new secret key
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        // Create a private key using the appropriate network kind
        let private_key = PrivateKey {
            compressed: true,
            network: self.network_kind,
            inner: secret_key,
        };

        // Generate the public key from the private key
        let public_key = PublicKey::from_private_key(&self.secp, &private_key);
        (private_key, public_key)
    }

    pub fn public_key_to_address(&self, public_key: &PublicKey) -> String {
        // Convert the public key to a Bitcoin address using the appropriate network (Bitcoin or Testnet)
        let address = bitcoin::Address::p2pkh(&*public_key, self.network);
        address.to_string()
    }
}
