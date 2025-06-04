# tendermint-rs

A minimal reimplementation of the Tendermint BFT consensus protocol in Rust.

Dependencies:

 * protocol buffers and gRPC - for serialisation and RPC.
 * tokio - for async runtime.
 * secp256k1 - for cryptographic identities.
 * serde - for message serialisation.
 * warp/reqwest - for HTTP server/clients (for node RPC).
 * hex.

## Usage.

## Readings.

 - [The latest gossip on BFT consensus.](https://arxiv.org/abs/1807.04938)
 - [Tendermint: Byzantine Fault Tolerance in the Age of Blockchains.](https://atrium.lib.uoguelph.ca/server/api/core/bitstreams/0816af2c-5fd4-4d99-86d6-ced4eef2fb52/content)

