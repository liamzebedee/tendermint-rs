use rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

struct ECDSAKeypair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl ECDSAKeypair {
    fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        ECDSAKeypair {
            secret_key,
            public_key,
        }
    }

    fn sign(&self, data: &[u8]) -> [u8; 65] {
        let secp = Secp256k1::new();
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let message = Message::from_slice(&hash).expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &self.secret_key);
        signature.serialize_compact()
    }

    fn verify(&self, data: &[u8], signature: &[u8; 65]) -> bool {
        let secp = Secp256k1::new();
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let message = Message::from_slice(&hash).expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &self.secret_key);
        secp.verify_ecdsa(&message, &signature, &self.public_key)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa() {
        // Create a new context for ECDSA operations
        let secp = Secp256k1::new();

        // Generate a new random keypair
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        // Data to be signed
        let data = b"Hello, Ethereum!";

        // Hash the data (Ethereum uses Keccak256)
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();

        // Sign the hash
        let message = Message::from_slice(&hash).expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &secret_key);

        // Verify the signature
        assert!(secp.verify_ecdsa(&message, &signature, &public_key).is_ok());

        println!("Signature verified successfully!");
    }
}
