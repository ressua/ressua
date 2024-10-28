mod wallet;
//mod transaction; 
mod network;
mod key_management;
mod storage;
mod utils;

use wallet::Wallet;

#[tokio::main]
async fn main() {

    // Mainnet or Testnet boolean parameter config
    let is_testnet = true;

    // Define wallet object
    let mut wallet = Wallet::new(is_testnet);
    
    // Generate a new address
    let address = wallet.generate_new_address(public_key);
    println!("New Address: {}", address);

    wallet.generate_and_save_key_pair();

    match wallet.get_balance(&address).await {
        Ok(balance) => println!("Balance for {}: {} BTC", address, balance),
        Err(e) => println!("Failed to get balance for {}: {}", address, e),
    }

}
