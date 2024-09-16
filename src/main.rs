use std::collections::VecDeque;

use tokio::sync::mpsc;

mod messages;
mod params;
mod process;
mod algos;

use crate::params::*;
use crate::process::*;

#[tokio::main]
async fn main() {
    // Create channels for each node
    let mut senders = Vec::new();
    let mut receivers = VecDeque::new();

    // Separate the creation of senders and receivers
    for _ in 0..NODES {
        let (tx, rx) = mpsc::channel(100);
        senders.push(tx);
        receivers.push_back(rx);
    }

    // Define proposer sequence (round-robin)
    let proposer_sequence: Vec<usize> = (0..NODES).collect();

    // Initialize nodes
    let mut nodes = Vec::new();
    for i in 0..NODES {
        let mut node_senders = Vec::new();
        for j in 0..NODES {
            if i != j {
                node_senders.push(senders[j].clone()); // Clone the sender only
            }
        }
        let receiver = receivers.pop_front().unwrap(); // Move the receiver
        let node = Process::new(i, receiver, node_senders, proposer_sequence.clone());
        nodes.push(node);
    }

    // Run all nodes
    let handles: Vec<_> = nodes
        .into_iter()
        .map(|mut node| {
            tokio::spawn(async move {
                node.run_epoch(None).await;
            })
        })
        .collect();

    // Wait for all nodes to finish
    for handle in handles {
        let _ = handle.await;
    }

    println!("Consensus reached.");
    // loop over all decisions from all nodes
    // nodes.iter().for_each(|node| {
    //     println!("Node {}: {:?}", node.id, node.decision);
    // });
}
