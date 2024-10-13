mod wallet;
mod transaction; 
mod network;
mod key_management;
mod storage;
mod utils;

use wallet::Wallet;

#[tokio::main]
async fn main() {

    // Mainnet or Testnet boolean parameter config
    let is_testnet = false;

    let mut wallet = Wallet::new(is_testnet);
    
    // Generate a new address
    let address = wallet.generate_new_address();
    println!("New Address: {}", address);

    // Fetch balance (example network function)
    //let balance = wallet.get_balance(&address).await;
    //println!("Balance for {}: {} BTC", address, balance);

    match wallet.get_balance(&address).await {
        Ok(balance) => println!("Balance for {}: {} BTC", address, balance),
        Err(e) => println!("Failed to get balance for {}: {}", address, e),
    }


    // Example transaction
    println!("Creating a transaction...");
    let utxos = vec![
        Utxo {
            outpoint: bitcoin::OutPoint::from_str("...").unwrap(),
            amount: 100000, // Example UTXO with 0.001 BTC (100,000 satoshis)
        },
    ];

    let recipient_address = "your-recipient-bitcoin-address";
    let send_amount = 50000; // Send 50,000 satoshis (0.0005 BTC)
    let change_address = "your-change-bitcoin-address";

    // Create a transaction
    let psbt = TransactionManager::create_transaction(
        utxos,
        recipient_address,
        send_amount,
        change_address
    );

    // Sign the transaction
    let private_key = PrivateKey::from_str("your-private-key-here").unwrap();
    TransactionManager::sign_transaction(&mut psbt, &private_key);

    // Broadcast the transaction
    TransactionManager::broadcast_transaction(&psbt).await;

}
