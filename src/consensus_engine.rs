use crate::protos::{ProposeMessage, VoteMessage};
use crate::protos::validator_service_client::ValidatorServiceClient;
use crate::crypto::Keypair;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use crate::config::{ConsensusConfig, ValidatorSetEntry};

pub enum ConsensusMessages {
    Proposal(ProposeMessage),
    Vote(VoteMessage),
}

pub struct ConsensusEngine {
    /// Our validator's keypair.
    pub validator_keypair: Keypair,

    /// Channel to receive messages from other validators.
    inbox: Arc<Mutex<mpsc::Receiver<ConsensusMessages>>>,
    
    /// Channels to send messages to other validators.
    outbox: Vec<mpsc::Sender<ConsensusMessages>>,

    /// Our consensus group (list of validators).
    group: HashMap<String, ValidatorServiceClient<Channel>>,
}


impl ConsensusEngine {
    /// Creates a new ConsensusEngine with the given keypair, inbox, outbox, and group.
    pub fn new(
        network_config: ConsensusConfig,
        validator_keypair: Keypair,
    ) -> Self {
        let (tx, rx) = mpsc::channel(10);
        let inbox = Arc::new(tokio::sync::Mutex::new(rx));
        let outbox = vec![tx];
        let group = std::collections::HashMap::new();

        ConsensusEngine {
            validator_keypair,
            inbox,
            outbox,
            group,
        }
    }

    /// Processes incoming messages from the inbox.
    pub async fn process_messages(&self) {
        let mut inbox = self.inbox.lock().await;
        while let Some(message) = inbox.recv().await {
            match message {
                ConsensusMessages::Proposal(propose_message) => {
                    // Handle proposal message
                    println!("Received proposal: {:?}", propose_message);
                }
                ConsensusMessages::Vote(vote_message) => {
                    // Handle vote message
                    println!("Received vote: {:?}", vote_message);
                }
            }
        }
    }
}

fn cs_run_round() {
    // get_validator_set_for_round
    // start_time = round_index * ROUND_LENGTH
    // proposer_for_round = ??
    // if proposer == me: yay. send proposal
    // wait for proposal till timeout
    // wait for prevotes until timeout
    // wait for precommits until timeout
    // decide value
    // emit Decision
    // return epochstate
}

// then in another place
// on decision -> ingest prevotes/precommits as block
// block processor -> map txs -> apply in order to get to state




// How to reconnect? How to architect sync?
// Once you are connected to validators you can proceed with consensus.
// value = nil
// vote nil
// precommit nil
// join next round
// value = x
// prevote/precommit x
// easy.
// just don't join live consensus until you have synced.
// sync is a separate background process.
// ie. download all blocks/decisions up until height
// you don't really need to have all decisions
// just blocks and proposals right? 
// you could probbaly implement that as:
// receive proposal
// if block.parent not known:
// sync until that point
// wait until next round
// yeah, that'd be easy.


pub fn get_validator_set_for_round(set_logs: Vec<ValidatorSetLog>, round: u64) -> Vec<ValidatorSetEntry> {
    let mut current_set = Vec::new();

    for log in set_logs {
        if log.from_height <= round {
            current_set = log.validators.clone();
        } else {
            break;
        }
    }

    current_set
}