mod wallet;
mod transaction;
mod network;
mod key_management;
mod storage;
mod utils;

use wallet::Wallet;

#[tokio::main]
async fn main() {
    let mut wallet = Wallet::new();
    
    // Generate a new address
    let address = wallet.generate_new_address();
    println!("New Address: {}", address);

    // Fetch balance (example network function)
    let balance = wallet.get_balance(&address).await;
    println!("Balance for {}: {} BTC", address, balance);
}
