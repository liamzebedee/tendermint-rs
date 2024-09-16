// Define message types
#[derive(Debug, Clone)]
pub enum Message {
    Propose { round: u64, value: String, from: usize },
    Prevote { round: u64, value: Option<String>, from: usize },
    Precommit { round: u64, value: Option<String>, from: usize },
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
