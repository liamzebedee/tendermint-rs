use std::collections::VecDeque;

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tendermint::{crypto::ECDSAKeypair, params::*, process::*};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;

async fn setup_pure_sendreceive() {
    // Create channels for each node
    let mut senders = Vec::new();
    let mut receivers = VecDeque::new();

    let get_value = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

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
                node_senders.push(senders[j].clone());
            }
        }
        let keypair = ECDSAKeypair::new();
        let receiver = receivers.pop_front().unwrap();
        let node = Process::new(
            i,
            keypair,
            Arc::new(Mutex::new(receiver)),
            node_senders,
            proposer_sequence.clone(),
            get_value,
        );
        nodes.push(node);
    }

    // Listen to events from node0.
    let mut subscriber1 = nodes[0].subscribe();
    tokio::spawn(async move {
        while let Some(event) = subscriber1.next().await {
            println!("Subscriber 1 received: {:?}", event);
        }
    });

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
}

#[tokio::main]
async fn main() {
    setup_pure_sendreceive().await;
}
