use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_stream::StreamExt;
use tendermint::crypto::ECDSAKeypair;
use tendermint::params::*;
use tendermint::process::*;
use tendermint::rpc_client::RpcClient;
use tendermint::rpc_server::Server;
use tendermint::messages::SignedMessage;

async fn setup_api_servers() {
    // Create channels for each node
    let mut senders = Vec::new();
    let mut receivers = VecDeque::new();

    let get_value = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    // Setup node API servers.
    for i in 0..NODES {
        let server = Server::<SignedMessage>::new(3030 + i as u16);
        receivers.push_back(server.get_receiver());
        let client = RpcClient::<SignedMessage>::new(
            100,
            format!("http://localhost:{}/inbox/", server.port),
        );
        senders.push(client.get_sender());

        tokio::spawn(async move {
            server.run().await;
        });
        tokio::spawn(async move {
            client.start().await;
        });
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
        let node =
            Process::new(i, keypair, receiver, node_senders, proposer_sequence.clone(), get_value);
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
        let _ = handle.await.unwrap();
    }

    println!("Consensus reached.");
}

#[tokio::main]
async fn main() {
    println!("Main");
    setup_api_servers().await;

    // just wait for sigkill
    tokio::signal::ctrl_c().await.unwrap();
}
