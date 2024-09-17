use rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use secp256k1::ecdsa::{SerializedSignature, Signature};

pub struct ECDSAKeypair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl ECDSAKeypair {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        ECDSAKeypair {
            secret_key,
            public_key,
        }
    }

    pub fn sign(&self, data: &[u8]) -> SerializedSignature {
        let secp = Secp256k1::new();
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let message = Message::from_slice(&hash).expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &self.secret_key);
        signature.serialize_der()
    }

    pub fn verify(&self, data: &[u8], signature: &SerializedSignature) -> bool {
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

