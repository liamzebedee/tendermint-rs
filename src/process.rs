use std::collections::HashMap;
use tokio::{
    sync::{mpsc, Mutex},
    time::{timeout, Duration},
};

use crate::{algos::*, events::*, messages::*, params::*};

#[derive(Debug, Clone)]
pub enum Event {
    Decision { height: u64, round: u64, value: String, from: usize },
}

/// A process running the Tendermint consensus algorithm.
pub struct Process {
    pub id: usize,

    /// Channel to receive messages from other processes.
    receiver: Mutex<mpsc::Receiver<Message>>,

    /// Channels to send messages to other processes.
    processes: Vec<mpsc::Sender<Message>>,
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
        receiver: mpsc::Receiver<Message>,
        processes: Vec<mpsc::Sender<Message>>,
        proposer_sequence: Vec<usize>,
        get_value: fn() -> String,
    ) -> Self {
        Process {
            id,
            receiver: Mutex::new(receiver),
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
            self.broadcast(Message::Propose { round, value: value.clone(), from: self.id }).await;
            // Save own proposal
            epoch.proposals.insert(round, value);
        }

        // Await proposals
        if self.id != proposer {
            let propose_timeout = get_timeout_for_round(round);
            let propose = self.receive_messages(round, MessageType::Propose, propose_timeout).await;
            if let Some(Message::Propose { round: r, value, from }) = propose {
                println!("Node {} received proposal from Node {}: {}", self.id, from, value);
                epoch.proposals.insert(r, value);
            } else {
                println!("Node {} did not receive proposal in round {}", self.id, round);
            }
        }

        // Prevote phase
        let proposal = epoch.proposals.get(&round).cloned();
        self.broadcast(Message::Prevote { round, value: proposal.clone(), from: self.id }).await;

        // Collect prevotes
        let prevote_timeout = get_timeout_for_round(round);
        let start = tokio::time::Instant::now();
        let mut prevotes = Vec::new();
        {
            let mut receiver = self.receiver.lock().await;

            while start.elapsed() < prevote_timeout {
                match timeout(prevote_timeout - start.elapsed(), receiver.recv()).await {
                    Ok(Some(Message::Prevote { round: r, value, from })) => {
                        if r == round {
                            prevotes.push(value.clone());
                            println!(
                                "Node {} received prevote from Node {}: {:?}",
                                self.id, from, value
                            );
                            if prevotes.len() >= QUORUM {
                                break;
                            }
                        }
                    }
                    Ok(Some(_)) => {
                        // Ignore other message types
                        continue;
                    }
                    _ => {
                        // Timeout reached or channel closed
                        break;
                    }
                }
            }
            epoch.prevotes.insert(round, prevotes.clone());
        }

        // Determine decision based on prevotes
        let decision = Self::majority_decision(&prevotes);
        println!("Node {} decided on {:?}", self.id, decision);
        self.broadcast(Message::Precommit { round, value: decision.clone(), from: self.id }).await;

        // Collect precommits
        let precommit_timeout = get_timeout_for_round(round);
        let mut precommits = Vec::new();
        let start = tokio::time::Instant::now();

        {
            let mut receiver = self.receiver.lock().await;
            while start.elapsed() < precommit_timeout {
                match timeout(precommit_timeout - start.elapsed(), receiver.recv()).await {
                    Ok(Some(Message::Precommit { round: r, value, from })) => {
                        if r == round {
                            precommits.push(value.clone());
                            println!(
                                "Node {} received precommit from Node {}: {:?}",
                                self.id, from, value
                            );
                            if precommits.len() >= QUORUM {
                                break;
                            }
                        }
                    }
                    Ok(Some(_)) => {
                        // Ignore other message types
                        continue;
                    }
                    _ => {
                        // Timeout reached or channel closed
                        break;
                    }
                }
            }
            epoch.precommits.insert(round, precommits.clone());
        }

        // Final decision
        if Self::count_occurrences(&precommits, &decision) >= QUORUM {
            println!("Node {} has committed value {:?} in round {}", self.id, decision, round);
            // Consensus reached
            epoch.decision = decision;
        } else {
            println!("Node {} failed to commit in round {}. Moving to next round.", self.id, round);
        }

        // wait 2s.
        // println!{"\n\n"};
        // tokio::time::sleep(Duration::from_secs(2)).await;

        epoch
    }

    async fn broadcast(&self, msg: Message) {
        for sender in &self.processes {
            let _ = sender.send(msg.clone()).await;
        }
    }

    async fn receive_messages(
        &self,
        _round: u64,
        msg_type: MessageType,
        duration: Duration,
    ) -> Option<Message> {
        while let Ok(Some(msg)) = timeout(duration, self.receiver.lock().await.recv()).await {
            if msg_type.matches(&msg) {
                return Some(msg);
            }
        }

        None
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
