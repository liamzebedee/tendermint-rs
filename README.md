# tendermint-rs

A minimal reimplementation of the Tendermint BFT consensus protocol in Rust.

Dependencies:

 * tokio.

## Status.

Protocol runs and achieves consensus, with rounds, epochs.

See [PLAN](./PLAN.md) for the backlog.

## Usage.

```sh
cargo build
```

## Readings.

 - [The latest gossip on BFT consensus.](https://arxiv.org/abs/1807.04938)
 - [Tendermint: Byzantine Fault Tolerance in the Age of Blockchains.](https://atrium.lib.uoguelph.ca/server/api/core/bitstreams/0816af2c-5fd4-4d99-86d6-ced4eef2fb52/content)

