// mod crypto;

use crate::crypto::{Signature, PublicKey, Keypair, verify_signature};
use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use serde_canonical_json::CanonicalFormatter;


// Define message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Propose { round: u64, value: String },
    Prevote { round: u64, value: Option<String> },
    Precommit { round: u64, value: Option<String> },
}

pub enum MessageType {
    Propose,
    Prevote,
    Precommit,
}

impl MessageType {
    pub fn matches(&self, msg: &Message) -> bool {
        match self {
            MessageType::Propose => matches!(msg, Message::Propose { .. }),
            MessageType::Prevote => matches!(msg, Message::Prevote { .. }),
            MessageType::Precommit => matches!(msg, Message::Precommit { .. }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignedMessage {
    pub body: Message,
    pub signature: Signature,
    pub sender: PublicKey,
}

impl SignedMessage {
    pub fn new(message: Message, keypair: &Keypair) -> Self {
        let sender = keypair.get_public_key();
        let sz: String = serde_json::to_string(&message).unwrap();
        let signature = keypair.sign(sz.as_bytes());

        SignedMessage { body: message, sender, signature }
    }

    pub fn verify(&self) -> bool {
        let sz: String = serde_json::to_string(&self.body).unwrap();
        verify_signature(sz.as_bytes(), &self.signature, self.sender)
    }

}

// Somehow:
// - nodes sign messages
// - nodes receive messages
// - messages which do not have a valid signature are discarded
