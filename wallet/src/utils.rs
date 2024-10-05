pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}
