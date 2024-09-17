use std::collections::VecDeque;

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;

mod algos;
mod crypto;
mod events;
mod messages;
mod params;
mod process;
mod rpc_client;
mod rpc_server;

use crypto::ECDSAKeypair;
use messages::SignedMessage;
use params::*;
use process::*;
use rpc_client::RpcClient;
use rpc_server::Server;

fn generate_node_keys() {
    for i in 0..NODES {
        let keypair = ECDSAKeypair::new();
        // Print pubkey,privkey in hex.
        println!(
            "Keypair {:?} pub={:?} prv={:?}",
            i,
            keypair.get_public_key().to_string(),
            keypair.get_secret_key().display_secret().to_string()
        );

        let keypair2 = ECDSAKeypair::new_from_privatekey(
            &keypair.get_secret_key().display_secret().to_string(),
        );
        println!(
            "Keypair {:?} pub={:?} prv={:?}",
            i,
            keypair2.get_public_key().to_string(),
            keypair2.get_secret_key().display_secret().to_string()
        );
    }
}

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

    // loop over all decisions from all nodes
    // nodes.iter().for_each(|node| {
    // })
    // print node and list of decisions
    // println!("Node {}: {:?}",
    // node.id,
    // node.get_decisions().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // });
}

async fn setup_api_servers() {
    // Create channels for each node
    let mut senders = Vec::new();
    let mut receivers = VecDeque::new();

    let get_value = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    // Setup node API servers.
    // let mut servers = Vec::new();
    for i in 0..NODES {
        let server = Server::<SignedMessage>::new(3030 + i as u16);
        receivers.push_back(server.get_receiver());
        let client = RpcClient::<SignedMessage>::new(
            100,
            format!("http://localhost:{}/inbox/", server.port),
        );
        senders.push(client.get_sender());
        // senders.push(server.get_sender());

        // servers.push(server.clone());
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
        let _ = handle.await;
    }

    println!("Consensus reached.");
}

#[tokio::main]
async fn main() {
    generate_node_keys();
    // setup_pure_sendreceive().await;
    setup_api_servers().await;

    // just wait for sigkill
    tokio::signal::ctrl_c().await.unwrap();
}
