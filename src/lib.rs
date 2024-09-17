pub mod algos;
pub mod events;
pub mod messages;
pub mod params;
pub mod process;
pub mod crypto;

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::verify_signature;

    #[test]
    fn test_create_sign() {
        let keypair = crypto::ECDSAKeypair::new();
        let data = b"gm tendermint";

        let signature = keypair.sign(data);
        assert!(verify_signature(data, &signature, keypair.get_public_key()));
        
        println!("Signature verified successfully!");
    }
}
