use crate::crypto::Keypair;
use crate::protos::{ProposeMessage, MessageType};
use chrono::{DateTime, Utc};
use sha3::{Keccak256, Digest};
use crate::crypto::{verify_signature, Signature, PublicKey};
use secp256k1::ecdsa::SerializedSignature;
use std::str::FromStr;

pub struct Proposal {
    inner: ProposeMessage,
}

impl Proposal {
    pub fn new(keypair: &Keypair, block: crate::protos::Block, round: i64, height: i64, timestamp: DateTime<Utc>) -> Self {
        let mut proposal = ProposeMessage {
            msg_type: MessageType::Propose as i32,
            round,
            height,
            value: Some(block),
            sig: Vec::new(),
            sender: keypair.get_public_key().to_string().as_bytes().to_vec(),
            timestamp: timestamp.timestamp(),
        };
        let sig_envelope = Self::sig_envelope(&proposal);
        let signature = keypair.sign(&sig_envelope);
        proposal.sig = serde_json::to_string(&signature).unwrap().as_bytes().to_vec();
        Self {
            inner: proposal,
        }
    }

    pub fn from_message(msg: ProposeMessage) -> Result<Self, String> {
        Ok(Self {
            inner: msg,
        })
    }

    pub fn sig_envelope(msg: &ProposeMessage) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(&msg.msg_type.to_le_bytes());
        hasher.update(&msg.round.to_le_bytes());
        hasher.update(&msg.height.to_le_bytes());
        if let Some(ref block) = msg.value {
            let block_hash = crate::types::Block::from_message(block.clone()).unwrap().hash();
            hasher.update(&block_hash);
        }
        hasher.update(&msg.sender);
        hasher.update(&msg.timestamp.to_le_bytes());
        hasher.finalize().to_vec()
    }

    pub fn sig_verify(&self) -> bool {
        let sig_envelope = Self::sig_envelope(&self.inner);
        let signature_string = String::from_utf8(self.inner.sig.clone()).unwrap();
        let signature = Signature::from_str(&signature_string).unwrap();
        let public_key_string = String::from_utf8(self.inner.sender.clone()).unwrap();
        let public_key = PublicKey::from_str(&public_key_string).unwrap();
        verify_signature(&sig_envelope, &signature.to_inner(), public_key)
    }

    pub fn validate(&self, clock: &impl Clock) -> bool {
        if !clock.is_recent(self.inner.timestamp) {
            return false;
        }
        self.sig_verify()
    }
}

pub trait Clock {
    fn is_recent(&self, timestamp: i64) -> bool;
} 