// Define the number of nodes and the maximum number of faulty nodes (f)
// NOTE: NODES must be at least 4 if you're building a real BFT system. N >= 3F+1.
pub const NODES: usize = 5;
pub const F: usize = 1;
pub const QUORUM: usize = 2 * F + 1;
pub const TIMEOUT_PROPOSE: u64 = 2000; // timeout for PROPOSE
pub const TIMEOUT_VOTE: u64 = 500; // timeouts for PREVOTE and PRECOMMIT
pub const ROUND_LENGTH_T: u64 = TIMEOUT_PROPOSE + TIMEOUT_VOTE * 2;