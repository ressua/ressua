use reqwest::Client;
use serde_json::Value;

pub struct BitcoinNetwork {
    client: Client,
    network: String, // Added to distinguish between mainnet and testnet
}

impl BitcoinNetwork {
    pub fn new(is_testnet: bool) -> Self {
        let network = if is_testnet {
            "https://mempool.space/testnet".to_string()
        } else {
            "https://mempool.space".to_string()
        };
        BitcoinNetwork {
            client: Client::new(),
            network,
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64, reqwest::Error> {
        let url = format!("{}/api/address/{}", self.network, address);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        
        // The "chain_stats" section contains confirmed and unconfirmed balances
        let balance_in_satoshis = json["chain_stats"]["funded_txo_sum"]
            .as_u64()
            .unwrap_or(0) - json["chain_stats"]["spent_txo_sum"]
            .as_u64()
            .unwrap_or(0);
        
        // Convert to BTC
        Ok(balance_in_satoshis as f64 / 100_000_000.0)
    }
}
