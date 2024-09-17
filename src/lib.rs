pub mod algos;
pub mod events;
pub mod messages;
pub mod params;
pub mod process;
pub mod crypto;

pub use crate::crypto::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sign() {
        let keypair = crypto::ECDSAKeypair::new();
        let data = b"gm tendermint";

        let signature = keypair.sign(data);
        assert!(keypair.verify(data, &signature));
        
        println!("Signature verified successfully!");
    }
}
