// Define the number of nodes and the maximum number of faulty nodes (f)
// NOTE: NODES must be at least 4 if you're building a real BFT system. N >= 3F+1. 
pub const NODES: usize = 5;
pub const F: usize = 1;
pub const QUORUM: usize = 2 * F + 1;