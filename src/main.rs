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

#[tokio::main]
async fn main() {
    generate_node_keys();
}
