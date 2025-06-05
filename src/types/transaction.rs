use crate::crypto::Keypair;
use crate::protos::Transaction as ProtoTransaction;
use sha3::{Sha3_256, Digest};

pub struct Transaction {
    inner: ProtoTransaction,
}

impl Transaction {
    pub fn new(keypair: &Keypair, data: Vec<u8>) -> Self {
        let mut tx = ProtoTransaction {
            data,
            sig: Vec::new(),
            sender: Vec::new(),
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
        hasher.update(&self.inner.data);
        hasher.finalize().to_vec()
    }

    pub fn sig_envelope(msg: &ProtoTransaction) -> Vec<u8> {
        // TODO: implement proper signature envelope
        // This should include all fields that need to be signed
        Vec::new()
    }

    pub fn sig_verify(&self) -> bool {
        // TODO: fix signature and public key types
        false
    }

    pub fn validate(&self) -> bool {
        // TODO: implement state machine validation
        // This should validate the transaction according to the state machine rules
        true
    }
} 