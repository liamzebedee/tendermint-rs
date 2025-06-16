use clap::Parser;
use serde_json::Result;
use std::{net::IpAddr, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use tokio_stream::StreamExt;
use crate::cli::utils::CmdAsync;
use crate::config::{parse_config, AccountConfig, ValidatorInfo};
use crate::crypto::ECDSAKeypair;
use crate::messages::SignedMessage;
use crate::process::Process;
use crate::rpc_server::Server;
use std::sync::Arc;

pub struct NodeOutput {}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct NodeArgs {
    // port
    #[clap(long, default_value = "3030")]
    port: u16,

    // host
    #[clap(long, default_value = "0.0.0.0")]
    host: IpAddr,

    // privatekey
    #[clap(long)]
    account: PathBuf,

    // genesis config.
    #[clap(long)]
    config: PathBuf,
}

impl CmdAsync for NodeArgs {
    type Output = Result<NodeOutput>;

    async fn run(self) -> Self::Output {
        let config = parse_config(self.config);
        // Load the account config.
        let accountData = std::fs::read_to_string(self.account).unwrap();
        let account: AccountConfig = serde_json::from_str(&accountData).unwrap();
        println!("Account: {:?}", account);
        run_node(config.validators, self.host, self.port).await;
        Ok(NodeOutput {})
    }
}

async fn run_node(_validators: Vec<ValidatorInfo>, host: IpAddr, port: u16) {
    // Network configuration:
    // - peers: (pubkey,address)[]
    // Parse the configuration file.

    // Setup RPC server.
    // Setup RPC client for each peer.
    // Setup process.
    // Run process.

    let keypair = ECDSAKeypair::new();

    let peer_senders = Vec::new();

    let addr = std::net::SocketAddr::new(host, port);
    let api_server = Server::new(addr);
    let receiver = api_server.get_receiver();
    tokio::spawn(async move {
        api_server.run().await;
    });
    // tokio::spawn(async move {
    //     client.start().await;
    // });

    // The function to get the current value for the chain.
    let get_value = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    // Define proposer sequence (round-robin)
    let proposer_sequence: Vec<usize> = (0..4).collect();
    let (signed_sender, signed_receiver) = tokio::sync::mpsc::channel(100);
    let signed_receiver = std::sync::Arc::new(tokio::sync::Mutex::new(signed_receiver));
    let mut process =
        Process::new(0, keypair, signed_receiver, peer_senders, proposer_sequence.clone(), get_value);

    // Listen to events from node0.
    let mut subscriber1 = process.subscribe();
    tokio::spawn(async move {
        while let Some(event) = subscriber1.next().await {
            println!("Subscriber 1 received: {:?}", event);
        }
    });

    tokio::spawn(async move {
        process.run_epoch(None).await;
    })
    .await
    .unwrap();

    println!("Consensus reached.");
}
