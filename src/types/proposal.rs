use crate::crypto::Keypair;
use crate::protos::{ProposeMessage, MessageType};
use chrono::{DateTime, Utc};

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
            sender: Vec::new(),
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

    pub fn sig_envelope(_msg: &ProposeMessage) -> Vec<u8> {
        Vec::new()
    }

    pub fn sig_verify(&self) -> bool {
        false
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