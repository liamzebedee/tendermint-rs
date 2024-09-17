use std::collections::VecDeque;
use tokio::sync::{mpsc, Mutex};
use std::{
    time::{SystemTime, UNIX_EPOCH},
};

use tokio_stream::StreamExt;
use tendermint::crypto::ECDSAKeypair;
use tendermint::messages::SignedMessage;
use tendermint::process::Process;
use tendermint::rpc_server::Server;

#[tokio::main]
async fn main() {
    // Network configuration:
    // - peers: (pubkey,address)[]

    // Setup RPC server.
    // Setup RPC client for each peer.
    // Setup process.
    // Run process.

    // let peers = [
    //     ("http://localhost:3001"),
    // ];

    let keypair = ECDSAKeypair::new();

    let mut peer_senders = Vec::new();

    let api_server = Server::<SignedMessage>::new(3030);
    let receiver = api_server.get_receiver();
    tokio::spawn(async move {
        api_server.run().await;
    });
    // tokio::spawn(async move {
    //     client.start().await;
    // });

    let get_value = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    // Define proposer sequence (round-robin)
    let proposer_sequence: Vec<usize> = (0..4).collect();
    let mut process =
        Process::new(0, keypair, receiver, peer_senders, proposer_sequence.clone(), get_value);

    // Listen to events from node0.
    let mut subscriber1 = process.subscribe();
    tokio::spawn(async move {
        while let Some(event) = subscriber1.next().await {
            println!("Subscriber 1 received: {:?}", event);
        }
    });

    tokio::spawn(async move {
        process.run_epoch(None).await;
    }).await.unwrap();

    println!("Consensus reached.");
}
