use crate::utils::CmdSync;
use clap::Parser;
use serde_json::Result;
use tendermint::{
    config::{TendermintConfig, ValidatorInfo},
    crypto::ECDSAKeypair,
};

pub struct NetworkOutput {}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct NetworkArgs {}

impl CmdSync for NetworkArgs {
    type Output = Result<NetworkOutput>;

    fn run(self) -> Self::Output {
        let keypair = ECDSAKeypair::new();
        let config = TendermintConfig {
            validators: vec![ValidatorInfo {
                pubkey: keypair.get_public_key().to_string(),
                address: "0.0.0.0".parse().unwrap(),
                port: 3030,
            }],
        };
        // Print to JSON format (pretty)
        let config = serde_json::to_string_pretty(&config).unwrap();
        println!("{}", config);
        Ok(NetworkOutput {})
    }
}
