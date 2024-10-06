Demo
====

The point of the demo is to demonstrate how Tendermint consensus works in the face of failures.

Ideally we will have a network of ~5 validators. So long as we receive 2f+1 votes then we are good.

A consensus system agrees on a single value ([single shot consensus](https://decentralizedthoughts.github.io/2022-11-19-from-single-shot-to-smr/)). In order to make this demo simple, we're going to agree to the current system time. This is a value that nodes can get from their system clock, and doesn't require any gossipping unlike transactions in a blockchain (replicated state machine) model.

A network begins at a particular genesis timestamp, and progresses at an irregular interval depending on the connectivity of the nodes. When a node disconnects (say it shuts down) and then reconnects to other nodes, it needs to figure out the current proposer rotation. Why? The current proposer is determined by the current round. Nodes can get in sync with each other node by simply listening for messages. 

## Questions.

**What happens when a node disconnects and then needs to reconnect?**
**Does the protocol continue when one node is offline? Or there are less than the required number of connections?**

> Proposer should not initiate Paxos if it cannot communicate with enough Acceptors to constitute a Quorum. 

Okay so that makes sense.

--

**What do they do to resync?**

 - the current time in the protocol. ie. the current phase (propose,vote,commit). 
 - the current height in the protocol. ie. in order to know which is the next proposer.
 - the current validator set in the protocol. ie. in order to know which of the validators could become a proposer.
 - the start time of the protocol?

**How do you know if the protocol has stalled ie. due to lack of validators?**
**How do you prevent an eclipse attack? ie. validators fool you into alternative view of consensus**

> https://vitalik.eth.limo/general/2020/11/06/pos2020.html

> See here for the original intro to the concept of "weak subjectivity". Essentially, the first time a node comes online, and any subsequent time a node comes online after being offline for a very long duration (ie. multiple months), that node must find some third-party source to determine the correct head of the chain. This could be their friend, it could be exchanges and block explorer sites, the client developers themselves, or many other actors. PoW does not have this requirement.




## Build.

```sh
cargo build --release
```

## Setup network.

```sh
cargo run network > genesis-config.json
cargo run account --new > account.json
cargo run node --config genesis-config.json --account account.json
```

## Install.

```sh
cargo run accounts --new
cargo run node --prvkey XXX --peers 
```


