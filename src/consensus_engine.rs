use crate::algos::{get_proposer_for_round, get_timeout_for_round};
use crate::params::{TIMEOUT_PROPOSE, TIMEOUT_VOTE, ROUND_LENGTH_T};
use crate::protos::{self, ProposeMessage, VoteMessage, MessageType as ProtoMessageType, Block as ProtoBlock};
use crate::protos::validator_service_client::ValidatorServiceClient;
use crate::crypto::{timestamp, Keypair};
use crate::types::vote::VoteType;
use crate::config::ValidatorSetLog;
use crate::types::{Proposal, Vote, Block};
use std::collections::HashMap;
use std::default;
use std::sync::Arc;
use std::time::Duration;
use chrono::round;
use futures::SinkExt;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use crate::config::{ConsensusConfig, ValidatorSetEntry};

#[derive(Debug, Clone)]
pub enum ConsensusMessages {
    Proposal(crate::protos::ProposeMessage),
    Vote(crate::protos::VoteMessage),
}

type Inbox = Arc<Mutex<mpsc::Receiver<ConsensusMessages>>>;
type Outbox = Vec<mpsc::Sender<ConsensusMessages>>;

pub struct ConsensusEngine {
    /// Our validator's keypair.
    pub validator_keypair: Keypair,

    /// Channel to receive messages from other validators.
    inbox: Inbox,
    
    /// Channels to send messages to other validators.
    outbox: Outbox,

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



// All you have to do:
// Implement this in a way which is stateless
// ie. pure timeouts

// /// Consensus operates in terms of epochs, which contain an unlimited number of rounds.
// #[derive(Debug, Clone)]
// pub struct EpochState {
//     /// The current height of the consensus instance.
//     height: u64,
//     /// The current round number.
//     round: u64,
//     /// Stores a proposal received for each round.
//     proposals: HashMap<u64, String>,
//     /// Stores prevote messages received for each round.
//     prevotes: HashMap<u64, Vec<Option<String>>>,
//     /// Stores precommit messages received for each round.
//     precommits: HashMap<u64, Vec<Option<String>>>,
//     /// The decision reached by the consensus algorithm, if any.
//     decision: Option<String>,
// }


// inputs: (inbox, outbox, roundid, networkconfig, getvalue fn, sync fn)
// outputs: decision(value: Option<T>)
async fn cs_run_round(
    inbox: Inbox,
    outbox: Outbox,
    keypair: Keypair,
    round_idx: u64,
    height: u64,
    network_config: ConsensusConfig,
    get_value: fn() -> Block
) -> Option<Vec<u8>> {
    // get_validator_set_for_round
    let validator_set = get_validator_set_for_round(network_config.validator_set_logs, round_idx);
    
    // start_time = round_index * ROUND_LENGTH
    let start_time = network_config.genesis_start_time + round_idx * ROUND_LENGTH_T;

    // proposer_for_round = ??
    let proposer = get_proposer_for_round(round_idx, validator_set);

    let mut proposal: Option<Proposal> = None;

    // if proposer == me; send proposal
    if proposer.public_key.to_bytes() == keypair.get_public_key().to_bytes() {
        let block = get_value().inner;
        let block_height = block.height;
        let my_proposal = Proposal::new(
            &keypair,
            block,
            round_idx + 1,
            block_height,
            timestamp()
        );
        for sender in outbox {
            let _ = sender.send(ConsensusMessages::Proposal(my_proposal.inner.clone())).await;
        }
        proposal = Some(my_proposal);
    } else {
        // Await proposals
        let propose_timeout = get_timeout_for_round(round_idx);
        tokio::time::sleep(propose_timeout).await;

        while let Some(message) = inbox.lock().await.recv().await {
            match message {
                ConsensusMessages::Proposal(propose_message) => {
                    if propose_message.round == round_idx {
                        println!(
                            "Node received proposal from Node {:?}",
                            propose_message.sender
                        );
                        proposal = Some(Proposal::from_message(propose_message));
                        break;
                        // Process the proposal message
                        // You might want to store or validate the proposal here
                    }
                }
                _ => {
                    // Handle other message types if necessary
                }
            }
        };
    }

    let proposal_hash: Vec<u8> = match proposal {
        Some(proposal) => {
            match proposal.inner.value {
                None => vec![],
                Some(proto_block) => {
                    let block = Block::from_message(proto_block);
                    block.hash()
                }
            }
        }
        None => vec![],
    };

    // validate proposal
        // if (proposal.block.parent not found) vote nil
    
    // emit prevote.
    let prevote = Vote::new(&keypair, VoteType::Prevote, height, round_idx, proposal_hash, timestamp());

    // wait for prevotes until timeout
    let prevote_timeout = Duration::from_millis(TIMEOUT_VOTE);
    tokio::time::sleep(prevote_timeout).await;

    // collect prevotes.
    let mut prevotes = Vec::new();
    while let Some(message) = inbox.lock().await.recv().await {
        match message {
            ConsensusMessages::Vote(vote_message) => {
                if vote_message.msg_type == protos::MessageType::Prevote as i32 && vote_message.round == round_idx {
                    println!(
                        "Node received vote from Node {:?}",
                        vote_message.sender
                    );
                    prevotes.push(vote_message);
                }
            }
            _ => {
                // Handle other message types if necessary
            }
        }
    };

    // Determine decision based on prevotes
    let decision = majority_decision(&prevotes);
    let my_precommit = Vote::new(
        &self.validator_keypair,
        VoteType::Precommit,
        height,
        round_idx,
        decision.unwrap_or_else(|| vec![]),
        timestamp()
    );
    // println!("Node {} decided on {:?}", self.id, decision);
    // Emit Precommit.


    // wait for precommits until timeout
    let precommit_timeout = Duration::from_millis(TIMEOUT_VOTE);
    tokio::time::sleep(precommit_timeout).await;

    // collect precommits.
    while let Some(message) = inbox.lock().await.recv().await {
        match message {
            ConsensusMessages::Vote(vote_message) => {
                if vote_message.msg_type == protos::MessageType::Precommit as i32 && vote_message.round == round_idx {
                    println!(
                        "Node received vote from Node {:?}",
                        vote_message.sender
                    );
                    prevotes.push(vote_message);
                }
            }
            _ => {
                // Handle other message types if necessary
            }
        }
    };

    // decide value
    // emit Decision
    // return epochstate

    None
}

fn cs_sync() {
    // in background
    // query peers for latest block height
    // download blocks
    // foreach block in order
        // get proposer for height
        // check correct proposer
        // get validator set for height
        // validate all signatures
        // valsigs = filter signatures to validator set
        // decision = majority(valsigs)
        // ingest block
        // add to blocklist (height, hash)
}


// then in another place
// on decision -> ingest prevotes/precommits as block
// block processor -> map txs -> apply in order to get to state

// how do you define this consensus engine?
// something that looks like:
// riscv vm
// write code
// deploy it into the vm




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

// async fn receive_messages_until_timeout(
//     msg_type: MessageType,
//     timeout_duration: Duration,
//     mut handler: impl FnMut(SignedMessage) -> bool,
//     on_timeout: impl Fn(),
// ) {
//     let start = tokio::time::Instant::now();
//     let mut receiver = self.receiver.lock().await;

//     while start.elapsed() < timeout_duration {
//         match timeout(timeout_duration - start.elapsed(), receiver.recv()).await {
//             Ok(Some(msg)) => {
//                 if !msg.verify() {
//                     // Ignore messages with invalid signatures.
//                     continue;
//                 }

//                 if msg_type.matches(&msg.body) && handler(msg) {
//                     break;
//                 }
//             }
//             _ => {
//                 // Timeout reached or channel closed
//                 on_timeout();
//                 break;
//             }
//         }
//     }
// }