use reqwest::Client;
use serde_json::Value;

pub struct BitcoinNetwork {
    client: Client,
}

impl BitcoinNetwork {
    pub fn new() -> Self {
        BitcoinNetwork {
            client: Client::new(),
        }
    }

    pub async fn get_balance(&self, address: &str) -> f64 {
        let url = format!("https://blockchain.info/q/addressbalance/{}?confirmations=6", address);
        let response = self.client.get(&url).send().await.unwrap();
        let balance_in_satoshis: u64 = response.text().await.unwrap().parse().unwrap();
        balance_in_satoshis as f64 / 100_000_000.0 // Convert to BTC
    }
}
