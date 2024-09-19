mod algos;
mod crypto;
mod events;
mod messages;
mod params;
mod process;
mod rpc_client;
mod rpc_server;

use crypto::ECDSAKeypair;
use params::*;

#[tokio::main]
async fn main() {
    generate_node_keys();
}
