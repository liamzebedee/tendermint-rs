use crate::crypto::Keypair;
use crate::protos::{VoteMessage, MessageType};
use crate::types::proposal::Clock;
use chrono::{DateTime, Utc};
use sha3::{Keccak256, Digest};
use crate::crypto::{verify_signature, Signature, PublicKey};
use secp256k1::ecdsa::SerializedSignature;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteType {
    Precommit,
    Prevote,
}

pub struct Vote {
    inner: VoteMessage,
}

impl Vote {
    pub fn new(keypair: &Keypair, vote_type: VoteType, height: i64, round: i64, value: Vec<u8>, timestamp: DateTime<Utc>) -> Self {
        let msg_type = match vote_type {
            VoteType::Prevote => MessageType::Prevote,
            VoteType::Precommit => MessageType::Precommit,
        };
        let mut vote = VoteMessage {
            msg_type: msg_type as i32,
            round,
            height,
            value,
            sig: Vec::new(),
            sender: keypair.get_public_key().to_string().as_bytes().to_vec(),
            timestamp: timestamp.timestamp(),
        };
        let sig_envelope = Self::sig_envelope(&vote);
        let signature = keypair.sign(&sig_envelope);
        vote.sig = serde_json::to_string(&signature).unwrap().as_bytes().to_vec();
        Self { inner: vote }
    }

    pub fn from_message(msg: VoteMessage) -> Self {
        Self { inner: msg }
    }

    pub fn get_type(&self) -> VoteType {
        match self.inner.msg_type {
            x if x == MessageType::Prevote as i32 => VoteType::Prevote,
            x if x == MessageType::Precommit as i32 => VoteType::Precommit,
            _ => panic!("Invalid vote type"),
        }
    }

    pub fn sig_envelope(msg: &VoteMessage) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(&msg.msg_type.to_le_bytes());
        hasher.update(&msg.round.to_le_bytes());
        hasher.update(&msg.height.to_le_bytes());
        hasher.update(&msg.value);
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