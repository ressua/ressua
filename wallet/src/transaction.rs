use bitcoin::psbt::Psbt;
use bitcoin::{Address, Transaction, TxIn, TxOut, Network};
use bitcoin::script::Script;
use bitcoin::consensus::encode::serialize;

use secp256k1::Secp256k1;
use secp256k1::SecretKey;
use bitcoin::key::PrivateKey;

use reqwest::Client;

pub struct Utxo {
    pub outpoint: bitcoin::OutPoint,  // Txid and vout (index)
    pub amount: u64,  // Amount in satoshis
}

pub struct TransactionManager;

impl TransactionManager {
    pub fn create_transaction(
        utxos: Vec<Utxo>,   // UTXO data with amounts, transaction IDs, etc.
        recipient_address: &str,  // Address to send BTC to
        send_amount: u64,  // Amount to send in satoshis
        change_address: &str  // Address to send the change back to
    ) -> Psbt {
        let mut inputs = vec![];
        let mut total_input_value = 0;

        // Collect inputs from available UTXOs
        for utxo in utxos {
            total_input_value += utxo.amount;
            inputs.push(TxIn {
                previous_output: utxo.outpoint,  // Txid and index
                script_sig: Script::new(),       // Placeholder; scriptSig is empty in PSBT
                sequence: 0xFFFFFFFF,            // Default sequence
                witness: vec![],                 // Placeholder; witness is empty in PSBT
            });

            if total_input_value >= send_amount {
                break;
            }
        }

        // Check if we have enough balance
        if total_input_value < send_amount {
            panic!("Not enough funds available in UTXOs.");
        }

        // Create the recipient output
        let recipient_output = TxOut {
            value: send_amount,
            script_pubkey: Address::from_str(recipient_address)
                .expect("Invalid recipient address")
                .script_pubkey(),
        };

        // Create the change output (send the change back to yourself)
        let change_value = total_input_value - send_amount;
        let change_output = TxOut {
            value: change_value,
            script_pubkey: Address::from_str(change_address)
                .expect("Invalid change address")
                .script_pubkey(),
        };

        // Create the transaction
        let tx = Transaction {
            version: 2,
            lock_time: 0,  // No timelock
            input: inputs,
            output: vec![recipient_output, change_output],
        };

        // Wrap it in a PSBT for signing
        let psbt = PartiallySignedTransaction {
            global: Default::default(),
            inputs: vec![Default::default(); tx.input.len()],
            outputs: vec![Default::default(); tx.output.len()],
        };

        psbt
    }

    pub fn sign_transaction(psbt: &mut Psbt, private_key: &PrivateKey) {
        let secp = Secp256k1::new();

        // Loop over each input and sign
        for input in &mut psbt.inputs {
            // Generate the signature for this input using the private key
            let sig_hash = psbt.global.unsigned_tx.signature_hash(
                input,
                &private_key.public_key(),
                bitcoin::SigHashType::All.as_u32(),
            );

            let signature = secp.sign(&sig_hash, &private_key.key);

            // Add the signature to the PSBT input
            input.partial_sigs.insert(
                private_key.public_key(),
                signature.serialize_der().to_vec(),
            );
        }
    }

    pub async fn broadcast_transaction(psbt: &Psbt) {
        // Convert PSBT to raw transaction
        let raw_tx = serialize(&psbt.global.unsigned_tx);

        // Use Blockstream API or any other provider to broadcast
        let client = Client::new();
        let url = "https://blockstream.info/api/tx";
        
        let response = client.post(url)
            .body(hex::encode(raw_tx))  // Convert raw tx to hex format
            .send()
            .await;

        match response {
            Ok(res) => println!("Transaction broadcasted: {:?}", res.text().await.unwrap()),
            Err(e) => println!("