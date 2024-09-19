Demo
====

The point of the demo is to demonstrate how Tendermint consensus works in the face of failures.

Ideally we will have a network of ~5 validators. So long as we receive 2f+1 votes then we are good.

A consensus system agrees on a single value ([single shot consensus](https://decentralizedthoughts.github.io/2022-11-19-from-single-shot-to-smr/)). In order to make this demo simple, we're going to agree to the current system time. This is a value that nodes can get from their system clock, and doesn't require any gossipping unlike transactions in a blockchain (replicated state machine) model.

A network begins at a particular genesis timestamp, and progresses at an irregular interval depending on the connectivity of the nodes. When a node disconnects (say it shuts down) and then reconnects to other nodes, it needs to figure out the current proposer rotation. Why? The current proposer is determined by the current round. Nodes can get in sync with each other node by simply listening for messages. 


## Build.

```sh
cargo build --release
```

## Install.

```sh
cargo run accounts --new
cargo run node --prvkey XXX --peers 
```


