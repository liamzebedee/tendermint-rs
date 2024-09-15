use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, Mutex};
use tokio::time::{timeout, Duration};

use crate::messages::*;
use crate::params::*;

// Define a Node
pub struct Process {
    id: usize,
    receiver: Mutex<mpsc::Receiver<Message>>,
    senders: Vec<mpsc::Sender<Message>>,
    current_round: Mutex<u64>,
    proposer_sequence: Vec<usize>,

    // State
    proposals: Mutex<HashMap<u64, String>>,
    prevotes: Mutex<HashMap<u64, Vec<Option<String>>>>,
    precommits: Mutex<HashMap<u64, Vec<Option<String>>>>,
}

impl Process {
    pub fn new(
        id: usize,
        receiver: mpsc::Receiver<Message>,
        senders: Vec<mpsc::Sender<Message>>,
        proposer_sequence: Vec<usize>,
    ) -> Self {
        Process {
            id,
            receiver: Mutex::new(receiver),
            senders,
            current_round: Mutex::new(0),
            proposer_sequence,
            proposals: Mutex::new(HashMap::new()),
            prevotes: Mutex::new(HashMap::new()),
            precommits: Mutex::new(HashMap::new()),
        }
    }

    pub async fn run(&self) {
        loop {
            let round = {
                let mut current_round = self.current_round.lock().await;
                *current_round += 1;
                *current_round
            };
            println!("Node {} starting round {}", self.id, round);

            // Determine proposer
            let proposer =
                self.proposer_sequence[(round - 1) as usize % self.proposer_sequence.len()];
            if self.id == proposer {
                // Propose a value (current timestamp)
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string();
                println!("Node {} proposing value {}", self.id, timestamp);
                self.broadcast(Message::Propose {
                    round,
                    value: timestamp.clone(),
                    from: self.id,
                })
                .await;
                // Save own proposal
                self.proposals.lock().await.insert(round, timestamp);
            }

            // Await proposals
            if self.id != proposer {
                let propose_timeout = Duration::from_millis(500);
                let propose = self
                    .receive_messages(round, MessageType::Propose, propose_timeout)
                    .await;
                if let Some(Message::Propose {
                    round: r,
                    value,
                    from,
                }) = propose
                {
                    println!(
                        "Node {} received proposal from Node {}: {}",
                        self.id, from, value
                    );
                    self.proposals.lock().await.insert(r, value);
                } else {
                    println!(
                        "Node {} did not receive proposal in round {}",
                        self.id, round
                    );
                }
            }
                

            // Prevote phase
            let proposal = self.proposals.lock().await.get(&round).cloned();
            self.broadcast(Message::Prevote {
                round,
                value: proposal.clone(),
                from: self.id,
            })
            .await;

            // Collect prevotes
            let prevote_timeout = Duration::from_millis(500);
            let mut prevotes = Vec::new();
            let mut receiver = self.receiver.lock().await;
            let start = tokio::time::Instant::now();

            while start.elapsed() < prevote_timeout {
                match timeout(prevote_timeout - start.elapsed(), receiver.recv()).await {
                    Ok(Some(Message::Prevote {
                        round: r,
                        value,
                        from,
                    })) => {
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
            self.prevotes.lock().await.insert(round, prevotes.clone());

            // Determine decision based on prevotes
            let decision = Self::majority_decision(&prevotes);
            println!("Node {} decided on {:?}", self.id, decision);
            self.broadcast(Message::Precommit {
                round,
                value: decision.clone(),
                from: self.id,
            })
            .await;

            // Collect precommits
            let precommit_timeout = Duration::from_millis(500);
            let mut precommits = Vec::new();
            receiver = self.receiver.lock().await;
            let start = tokio::time::Instant::now();

            while start.elapsed() < precommit_timeout {
                match timeout(precommit_timeout - start.elapsed(), receiver.recv()).await {
                    Ok(Some(Message::Precommit {
                        round: r,
                        value,
                        from,
                    })) if r == round => {
                        precommits.push(value.clone());
                        println!(
                            "Node {} received precommit from Node {}: {:?}",
                            self.id, from, value
                        );
                        if precommits.len() >= QUORUM {
                            break;
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
            self.precommits
                .lock()
                .await
                .insert(round, precommits.clone());

            // Final decision
            if Self::count_occurrences(&precommits, &decision) >= QUORUM {
                println!(
                    "Node {} has committed value {:?} in round {}",
                    self.id, decision, round
                );
                break; // Consensus reached
            } else {
                println!(
                    "Node {} failed to commit in round {}. Moving to next round.",
                    self.id, round
                );
            }
        }
    }

    async fn broadcast(&self, msg: Message) {
        for sender in &self.senders {
            let _ = sender.send(msg.clone()).await;
        }
    }

    async fn receive_messages(
        &self,
        round: u64,
        msg_type: MessageType,
        duration: Duration,
    ) -> Option<Message> {
        match timeout(duration, self.receiver.lock().await.recv()).await {
            Ok(Some(msg)) => {
                if msg_type.matches(&msg) {
                    Some(msg)
                } else {
                    None
                }
            }
            _ => None,
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
