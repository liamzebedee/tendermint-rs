#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{Keypair, PublicKey};
    use crate::validator_node::ValidatorNode;
    use crate::consensus_engine::*;
    use crate::config::*;
    use tokio::sync::mpsc;
    use std::sync::Arc;

    fn generate_validator_config(num_validators: usize) -> Vec<ValidatorConfig> {
        let mut validators = Vec::new();

        for i in 0..num_validators {
            let keypair = Keypair::new();
            let ip_port = format!("127.0.0.1:{}", 23000 + i); // Assigning IP:Port combos

            validators.push(ValidatorConfig {
                public_key: keypair.get_public_key(),
                address: ip_port,
            });
        }

        // Print the generated validators for verification
        for (i, validator) in validators.iter().enumerate() {
            println!("Validator {}: Public Key: {:?}, IP:Port: {}", i + 1, validator.public_key, validator.address);
        }

        validators
    }

    fn generate_network_config(genesis_start_time: u64, validators: &Vec<ValidatorConfig>) -> ConsensusConfig {
        let validator_set_logs = validators.iter().map(|validator| {
            ValidatorSetLog {
                from_height: 0,
                validators: vec![ValidatorSetEntry {
                    public_key: validator.public_key.clone(),
                    address: validator.address.clone(),
                }],
            }
        }).collect();

        ConsensusConfig {
            genesis_start_time,
            validator_set_logs: validator_set_logs
        }
    }

    #[tokio::test]
    async fn test_everything() {
        // 1. Generate validators.
        let validators = generate_validator_config(5);
        
        // 2. Generate validator config for the network.
        let network_config = generate_network_config(
            chrono::Utc::now().timestamp() as u64,
            &validators
        );
        println!("Generated Network Config: {:?}", network_config);

        // 3. Create a validator node for config#0 and start it.
        // 

        // 1. Validator Service.
        let validator_node = ValidatorNode {};
        let addr = validators[0].address.clone(); // Use the address from the validator config
        let server_handle = tokio::spawn(async move {
            validator_node.run_service(addr).await.unwrap();
        });

        // 2. Consensus engine.
        // Create mock/default arguments for ConsensusEngine
        let validator_keypair = Keypair::new();
        let consensus_engine = ConsensusEngine::new(
            network_config,
            validator_keypair
        );

        // Wait for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // You can add client code here to interact with the server

        // ideas:
        // - solution for "clock drift" on tendermint node reconnect
            // liveness messages
            // ask other peers for their clock, take median
        // - fixed block time modification
            // start block - starts at time t
            // timeouts are fixed and static
            // add up timeouts - (propsal, prevote, precommit) = 1 timestep
        // - leader-based system. ie. elect whenever a new master lease. and then master holds lease for predefined period.

        // 1. copy-paste existing process + get it running for 5 validators proposing messages over grpc
        // 1. edit so it has:
            // - startup
                // wait for connection to other validators
                // once its ready, then sync
                // sync will download decisions from one randomly chosen validator
                    // maintain validator set history
                    // if decisions > quorum, then ingest block. do not emit decision.
            // live mode
                // orient in round. need a sense of timing.
                    // wait until you receive precommit message
                    // then calculate the next step as:
                        // proposal.timestamp + TIMEOUT
                // get proposal
                    // validate etc.
                // ingest prevote
                // ingest precommit
        // 

        // Shutdown the server
        server_handle.abort();
    }
}
