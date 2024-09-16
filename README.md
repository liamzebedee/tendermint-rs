# tendermint-rs

A minimal reimplementation of the Tendermint BFT consensus protocol in Rust.

Dependencies:

 * tokio - for async runtime.
 * secp256k1 - for cryptographic identities.


## Status.

Protocol runs and achieves consensus, with rounds, epochs.

See [PLAN](./PLAN.md) for the backlog.


## Usage.

### Running tests.

```sh
cargo build
```

### Using it.

Rust Tendermint can be used to build a consistent and partition-tolerant network, with a custom value that is agreed per epoch, and event streams which allow you to consume different events (such agreement - referred to as a decision, and intermediate stages).

```rs
use tendermint::Process;

fn example() {
    // Setup networking substrate between nodes.
    let node = Process::new();
    
    // Pass a get_value callback to get the next candidate value for proposal.
    // Subscribe and listen to Decision events to process them.
}
```

See `examples/`.


## Readings.

 - [The latest gossip on BFT consensus.](https://arxiv.org/abs/1807.04938)
 - [Tendermint: Byzantine Fault Tolerance in the Age of Blockchains.](https://atrium.lib.uoguelph.ca/server/api/core/bitstreams/0816af2c-5fd4-4d99-86d6-ced4eef2fb52/content)

