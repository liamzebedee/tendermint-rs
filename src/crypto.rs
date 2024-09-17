use rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use secp256k1::ecdsa::{SerializedSignature};
use std::str::FromStr;

pub type Signature = SerializedSignature;
pub type PublicKey = secp256k1::PublicKey;
pub type Keypair = ECDSAKeypair;

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

    pub fn new_from_privatekey(private_key: &str) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        ECDSAKeypair {
            secret_key: secret_key,
            public_key,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn get_secret_key(&self) -> SecretKey {
        self.secret_key.clone()
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
}

pub fn verify_signature(data: &[u8], signature: &SerializedSignature, public_key: PublicKey) -> bool {
    let secp: Secp256k1<secp256k1::All> = Secp256k1::new();
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let message = Message::from_slice(&hash).expect("32 bytes");
    secp.verify_ecdsa(&message, &signature.to_signature().unwrap(), &public_key)
        .is_ok()
}