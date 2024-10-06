use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// The public key of the validator.
    pub pubkey: String,
    /// The IP address of the validator.
    pub address: IpAddr,
    /// The IP port of the validator.
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TendermintConfig {
    /// The set of validators at genesis.
    pub validators: Vec<ValidatorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub pubkey: String,
    pub privkey: String,
}

pub fn parse_config(config: PathBuf) -> TendermintConfig {
    // Parse the configuration file.
    let config = std::fs::read_to_string(config).unwrap();
    let config: TendermintConfig = serde_json::from_str(&config).unwrap();
    println!("Config: {:?}", config);
    config
}
