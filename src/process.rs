use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    time::{timeout, Duration},
};

use crate::{algos::*, crypto::*, events::*, messages::*, params::*};

#[derive(Debug, Clone)]
pub enum Event {
    Decision { height: u64, round: u64, value: String, from: usize },
}

/// A process running the Tendermint consensus algorithm.
pub struct Process {
    pub id: usize,

    pub keypair: Keypair,

    /// Channel to receive messages from other processes.
    receiver: Arc<Mutex<mpsc::Receiver<SignedMessage>>>,

    /// Channels to send messages to other processes.
    processes: Vec<mpsc::Sender<SignedMessage>>,
    proposer_sequence: Vec<usize>,

    /// Event source.
    events: EventSystem<Event>,

    /// State.
    decisions: Vec<String>,

    /// Callback to get the value to be proposed for agreement.
    get_value: fn() -> String,
}

/// Consensus operates in terms of epochs, which contain an unlimited number of rounds.
#[derive(Debug, Clone)]
pub struct EpochState {
    height: u64,
    round: u64,
    proposals: HashMap<u64, String>,
    prevotes: HashMap<u64, Vec<Option<String>>>,
    precommits: HashMap<u64, Vec<Option<String>>>,
    decision: Option<String>,
}

impl Process {
    pub fn new(
        id: usize,
        keypair: Keypair,
        receiver: Arc<Mutex<mpsc::Receiver<SignedMessage>>>,
        processes: Vec<mpsc::Sender<SignedMessage>>,
        proposer_sequence: Vec<usize>,
        get_value: fn() -> String,
    ) -> Self {
        Process {
            id,
            keypair,
            receiver,
            processes,
            proposer_sequence,
            decisions: Vec::new(),
            events: EventSystem::new(),
            get_value,
        }
    }

    /// Subscribes to the consensus event stream.
    pub fn subscribe(&self) -> impl tokio_stream::Stream<Item = Event> {
        self.events.subscribe()
    }

    /// Runs
    // pub async fn run(&self) {
    //     loop {
    //         epoch_state = self.run_round(epoch_state).await;
    //         if epoch_state.decision.is_some() {
    //             break;
    //         }
    //     }
    //     epoch_state
    // }

    /// Runs a single epoch of Tendermint consensus, taking in optionally the current epoch state.
    /// Each epoch consists of at least one round. If the round fails to reach consensus, the epoch
    /// will continue to the next round. This function returns upon the consensus deciding a new
    /// value.
    pub async fn run_epoch(&mut self, epoch_state: Option<EpochState>) -> EpochState {
        let mut epoch_state = epoch_state.unwrap_or(EpochState {
            height: 0,
            round: 0,
            proposals: HashMap::new(),
            prevotes: HashMap::new(),
            precommits: HashMap::new(),
            decision: None,
        });
        loop {
            epoch_state = self.run_round(epoch_state).await;

            if epoch_state.decision.is_some() {
                epoch_state.height += 1;
                self.decisions.push(epoch_state.decision.clone().unwrap());

                // Publish decision event
                self.events.publish(Event::Decision {
                    height: epoch_state.height,
                    round: epoch_state.round,
                    value: epoch_state.decision.clone().unwrap(),
                    from: self.id,
                });
                break;
            }
        }
        println!("Node {} decided on {:?}", self.id, epoch_state.decision);
        epoch_state
    }

    /// Runs a single round of Tendermint consensus, taking in the current epoch state.
    /// Returns the updated epoch state.
    pub async fn run_round(&self, epoch_state0: EpochState) -> EpochState {
        let mut epoch = epoch_state0.clone();
        epoch.round += 1;

        let round = epoch.round;
        println!("Node {} starting round {}", self.id, round);

        // Determine proposer
        let proposer = get_proposer_for_round(round as u8, &self.proposer_sequence);
        if self.id == proposer {
            // Propose a value.
            let value = (self.get_value)();
            println!("Node {} proposing value {}", self.id, value);
            self.broadcast(Message::Propose { round, value: value.clone() }).await;
            // Save own proposal
            epoch.proposals.insert(round, value);
        }

        // Await proposals
        if self.id != proposer {
            let propose_timeout = get_timeout_for_round(round);

            self.receive_messages_until_timeout(
                MessageType::Propose,
                propose_timeout,
                |msg| {
                    if let Message::Propose { round: r, value } = msg.body {
                        if r == round {
                            println!(
                                "Node {} received proposal from Node {}: {}",
                                self.id, msg.sender, value
                            );
                            epoch.proposals.insert(r, value);
                            return true
                        }
                    }
                    false
                },
                || {
                    // Timeout reached
                    println!("Node {} timed out waiting for proposals in round {}", self.id, round);
                },
            )
            .await;
        }

        // Prevote phase
        let proposal = epoch.proposals.get(&round).cloned();
        self.broadcast(Message::Prevote { round, value: proposal.clone() }).await;

        // Collect prevotes
        let prevote_timeout = get_timeout_for_round(round);
        let _start = tokio::time::Instant::now();
        let mut prevotes = Vec::new();

        self.receive_messages_until_timeout(
            MessageType::Prevote,
            prevote_timeout,
            |msg| {
                if let Message::Prevote { round: r, value } = msg.body {
                    if r == round {
                        prevotes.push(value.clone());
                        println!(
                            "Node {} received prevote from Node {}: {:?}",
                            self.id, msg.sender, value
                        );
                        if prevotes.len() >= QUORUM {
                            return true;
                        }
                    }
                }
                false
            },
            || {
                // Timeout reached
                println!("Node {} timed out waiting for prevotes in round {}", self.id, round);
            },
        )
        .await;
        epoch.prevotes.insert(round, prevotes.clone());

        // Determine decision based on prevotes
        let decision = Self::majority_decision(&prevotes);
        // println!("Node {} decided on {:?}", self.id, decision);
        self.broadcast(Message::Precommit { round, value: decision.clone() }).await;

        // Collect precommits
        let _precommit_timeout = get_timeout_for_round(round);
        let mut precommits = Vec::new();

        self.receive_messages_until_timeout(
            MessageType::Precommit,
            prevote_timeout,
            |msg| {
                if let Message::Precommit { round: r, value } = msg.body {
                    if r == round {
                        precommits.push(value.clone());
                        println!(
                            "Node {} received precommit from Node {}: {:?}",
                            self.id, msg.sender, value
                        );
                        if precommits.len() >= QUORUM {
                            return true;
                        }
                    }
                }
                false
            },
            || {
                // Timeout reached
                println!("Node {} timed out waiting for precommits in round {}", self.id, round);
            },
        )
        .await;
        epoch.precommits.insert(round, precommits.clone());

        // Final decision
        if Self::count_occurrences(&precommits, &decision) >= QUORUM {
            println!("Node {} has committed value {:?} in round {}", self.id, decision, round);
            // Consensus reached
            epoch.decision = decision;
        } else {
            println!("Node {} failed to decide in round {}. Moving to next round.", self.id, round);
        }

        epoch
    }

    async fn broadcast(&self, msg: Message) {
        let signed_msg = SignedMessage::new(msg, &self.keypair);
        for sender in &self.processes {
            let _ = sender.send(signed_msg.clone()).await;
        }
    }

    async fn receive_messages_until_timeout(
        &self,
        msg_type: MessageType,
        timeout_duration: Duration,
        mut handler: impl FnMut(SignedMessage) -> bool,
        on_timeout: impl Fn(),
    ) {
        let start = tokio::time::Instant::now();
        let mut receiver = self.receiver.lock().await;

        while start.elapsed() < timeout_duration {
            match timeout(timeout_duration - start.elapsed(), receiver.recv()).await {
                Ok(Some(msg)) => {
                    if !msg.verify() {
                        // Ignore messages with invalid signatures.
                        continue;
                    }

                    if msg_type.matches(&msg.body) && handler(msg) {
                        break;
                    }
                }
                _ => {
                    // Timeout reached or channel closed
                    on_timeout();
                    break;
                }
            }
        }
    }

    fn majority_decision(prevotes: &Vec<Option<String>>) -> Option<String> {
        let mut counts = HashMap::new();
        for vote in prevotes {
            *counts.entry(vote.clone()).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .filter(|&(_, count)| count >= QUORUM)
            .map(|(value, _)| value)
            .unwrap_or(None)
    }

    fn count_occurrences(precommits: &Vec<Option<String>>, decision: &Option<String>) -> usize {
        precommits.iter().filter(|&v| v == decision).count()
    }
}
