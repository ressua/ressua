use bitcoin::key::{PrivateKey, PublicKey};
use bitcoin::network::Network;
use bitcoin::NetworkKind;
use secp256k1::{rand, Secp256k1, SecretKey};

pub struct KeyManager {
    secp: Secp256k1<secp256k1::All>,
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager {
            secp: Secp256k1::new(),
        }
    }

    pub fn generate_key_pair(&self) -> (PrivateKey, PublicKey) {
        //let (secret_key, public_key) = generate_keypair(&mut rand::thread_rng());
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let private_key = PrivateKey {
            compressed: true,
            network: NetworkKind::Main, // Main or Test
            inner: secret_key,
        };
        let public_key = PublicKey::from_private_key(&self.secp, &private_key);
        (private_key, public_key)
    }

    pub fn public_key_to_address(&self, public_key: &PublicKey) -> String {
        // Convert public key to Bitcoin address (using P2PKH, for example)
        let address = bitcoin::Address::p2pkh(&*public_key, Network::Bitcoin); // Bitcoin or Testnet
        address.to_string()
    }
}
