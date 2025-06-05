use crate::protos::{ProposeMessage, VoteMessage};
use crate::protos::validator_service_client::ValidatorServiceClient;
use crate::crypto::Keypair;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tonic::transport::Channel;

enum ConsensusMessages {
    Proposal(ProposeMessage),
    Vote(VoteMessage),
}

struct ConsensusEngine {
    /// Our validator's keypair.
    pub validator_keypair: Keypair,

    /// Channel to receive messages from other validators.
    inbox: Arc<Mutex<mpsc::Receiver<ConsensusMessages>>>,
    
    /// Channels to send messages to other validators.
    outbox: Vec<mpsc::Sender<ConsensusMessages>>,

    /// Our consensus group (list of validators).
    group: HashMap<String, ValidatorServiceClient<Channel>>,
}

