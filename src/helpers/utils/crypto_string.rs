pub mod crypto_fn {
    // --------------- //
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    // --------------- //
    pub fn crypt_string_sha256 (charters: String) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&charters);
        hasher.result_str()
    }
}
