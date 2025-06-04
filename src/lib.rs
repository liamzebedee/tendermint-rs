pub mod algos;
pub mod cli;
pub mod clock;
pub mod config;
pub mod crypto;
pub mod events;
pub mod messages;
pub mod params;
pub mod process;
pub mod protos;
pub mod proposer_choice;
pub mod rpc_client;
pub mod rpc_server;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::verify_signature;

    #[test]
    fn test_create_sign() {
        let keypair = crypto::ECDSAKeypair::new();
        let data = b"gm tendermint";

        let signature = keypair.sign(data);
        assert!(verify_signature(data, &signature.to_inner(), keypair.get_public_key()));

        println!("Signature verified successfully!");
    }
}
