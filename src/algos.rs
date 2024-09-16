use tokio::time::{Duration};

pub fn get_proposer_for_round(round: u8, proposer_sequence: &[usize]) -> usize {
    proposer_sequence[(round - 1) as usize % proposer_sequence.len()]
}

/// Gets the timeout for a round.
/// Timeout in Tendermint increases exponentially with round number, in order to give more time for nodes to reach consensus in the presence of delays.
pub fn get_timeout_for_round(round: u64) -> Duration {
    Duration::from_millis(1000)
}