use crate::crypto::Keypair;
use crate::protos::Transaction as ProtoTransaction;
use sha3::{Sha3_256, Digest, Keccak256};
use crate::crypto::verify_signature;
use secp256k1::ecdsa::SerializedSignature;
use std::str::FromStr;

pub struct Transaction {
    inner: ProtoTransaction,
}

impl Transaction {
    pub fn new(keypair: &Keypair, data: Vec<u8>) -> Self {
        let mut tx = ProtoTransaction {
            data,
            sig: Vec::new(),
            sender: keypair.get_public_key().to_string().as_bytes().to_vec(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        // Sign the transaction
        let sig_envelope = Self::sig_envelope(&tx);
        let signature = keypair.sign(&sig_envelope);
        tx.sig = serde_json::to_string(&signature).unwrap().as_bytes().to_vec();

        Self { inner: tx }
    }

    pub fn from_message(msg: ProtoTransaction) -> Self {
        Self { inner: msg }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        let sig_envelope = Self::sig_envelope(&self.inner);
        hasher.update(&sig_envelope);
        hasher.finalize().to_vec()
    }

    pub fn sig_envelope(msg: &ProtoTransaction) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(&msg.data);
        hasher.update(&msg.sender);
        hasher.update(&msg.timestamp.to_le_bytes());
        hasher.finalize().to_vec()
    }

    pub fn sig_verify(&self) -> bool {
        let sig_envelope = Self::sig_envelope(&self.inner);
        let signature_string = String::from_utf8(self.inner.sig.clone()).unwrap();
        let signature = crate::crypto::Signature::from_str(&signature_string).unwrap();
        let public_key_string = String::from_utf8(self.inner.sender.clone()).unwrap();
        let public_key = crate::crypto::PublicKey::from_str(&public_key_string).unwrap();
        verify_signature(&sig_envelope, &signature.to_inner(), public_key)
    }

    pub fn validate(&self) -> bool {
        // TODO: implement state machine validation
        // This should validate the transaction according to the state machine rules
        true
    }
} 