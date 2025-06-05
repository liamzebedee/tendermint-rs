use crate::crypto::Keypair;
use crate::protos::{VoteMessage, MessageType};
use crate::types::proposal::Clock;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteType {
    Precommit,
    Prevote,
}

pub struct Vote {
    inner: VoteMessage,
}

impl Vote {
    pub fn new(keypair: &Keypair, vote_type: VoteType, height: i64, round: i64, timestamp: DateTime<Utc>) -> Self {
        let msg_type = match vote_type {
            VoteType::Prevote => MessageType::Prevote,
            VoteType::Precommit => MessageType::Precommit,
        };
        let mut vote = VoteMessage {
            msg_type: msg_type as i32,
            round,
            height,
            value: Vec::new(),
            sig: Vec::new(),
            sender: Vec::new(),
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

    pub fn sig_envelope(_msg: &VoteMessage) -> Vec<u8> {
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