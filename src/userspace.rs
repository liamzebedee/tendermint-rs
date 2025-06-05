#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{Keypair, PublicKey};
    use crate::validator_node::ValidatorNode;
    use tokio::sync::mpsc;
    use std::sync::Arc;

    #[derive(Debug)]
    struct ValidatorConfig {
        public_key: PublicKey,
        address: String, // ip:port
    }

    #[derive(Debug)]
    struct ValidatorSetEntry {
        public_key: PublicKey,
        address: String,
    }

    #[derive(Debug)]
    struct ValidatorSetLog {
        from_height: u64,
        validators: Vec<ValidatorSetEntry>,
    }

    #[derive(Debug)]
    struct ConsensusConfig {
        validator_set_logs: Vec<ValidatorSetLog>
    }

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

    fn generate_network_config(validators: &Vec<ValidatorConfig>) -> ConsensusConfig {
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
            validator_set_logs: validator_set_logs
        }
    }

    #[tokio::test]
    async fn test_everything() {
        // 1. Generate validators.
        let validators = generate_validator_config(5);
        
        // 2. Generate validator config for the network.
        let network_config = generate_network_config(&validators);
        println!("Generated Network Config: {:?}", network_config);

        // 3. Create a validator node for config#0 and start it.
        let validator_node = ValidatorNode {};
        let addr = validators[0].address.clone(); // Use the address from the validator config
        let server_handle = tokio::spawn(async move {
            validator_node.run_service(addr).await.unwrap();
        });

        // Wait for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // You can add client code here to interact with the server

        // Shutdown the server
        server_handle.abort();
    }
}
