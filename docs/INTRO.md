
In this article I'm going to explain my recent efforts in reimplementing Tendermint from scratch.

## Overview.

Tendermint is a byzantine fault tolerant consensus protocol. 

The network is 3F+1 nodes, with the allowance for F byzantine failures. In the base case of F=1, the network must have 4 nodes. 

The network decides on a single value via the consensus algorithm, at an interval bounded by a timeout. 

The network communication cost is $O(N^2)$, as each node in the network must gossip to every other node.

The core of Tendermint's algorithm:

 * Tendermint is used to build blockchains. 
 * The consensus protocol produces decisions - each successful agreement on a value among the nodes is called a decision.
 * Each decision happens within an epoch. 
 * During an epoch, the network attempts to decide on a value. 
 * An epoch consists of an unlimited number of rounds.
 * A round consists of 3 phases: propose, prevote, precommit. 
   * **Propose**. A single proposer is chosen deterministically from the validator set. The proposer sends a proposed value to decide on to the rest of the network. The rest of nodes wait a predefined timeout for this proposal.
   * **Prevote**. In the prevote phase, nodes try to agree on which value was proposed, and in the next phase they agree on whether to commit it. Every node sends votes to every other node, stating the value they believe is proposed for this round. Upon 2f+1 prevotes for a value, a node sets the decision value.
   * **Precommit**. In the precommit phase, nodes try to commit to the decision value they agreed on in the previous phase. We broadcast a precommit for the decision value we've seen (or nil). Upon 2f+1 precommits for a decision value, we decide on it. 
   * **Decision**. If we have received 2f+1 precommits for a non-nil value, we have achieved a decision and finish this epoch. Else, we begin the next round, which selects the next proposer. 
 * The protocol exhibits safety, finality, liveness. 
   * **Timeouts**.
      >**FLP impossibility result** - it is impossible in a fully asynchronous message-passing distributed system, in which at least one process may have a crash failure, for a deterministic algorithm to achieve consensus
   * Tendermint eventually finalises due to timeouts. At each phase of a round - propose, prevote, precommit - timeouts are used to make sure the protocol completes. 
   * Tendermint guarantees the safety and liveness of the blockchain so long as less than 1/3 of the total weight of the Validator set is malicious or faulty.

In terms of Tendermint's implementation:

 * Sync.
   * To participate in consensus rounds, we need to be fully synced in order to determine the current proposer.
   * Unlike Nakamoto consensus, Tendermint consensus history is linear. There are no divergent branches to explore, as the protocol finalises on only one block at each consensus epoch. So this makes sync a lot easier.
   * **Sync algorithm**. A node connects to all available peers and requests their heights. Starting from our latest pool.height, we request blocks in sequence from peers that reported higher heights than ours. Requests are continuously made for blocks of higher heights until the limit is reached. The node is considered caught up if it has at least one peer and its height is at least as high as the max reported peer height.
   * Once we have completed synchronisation, the node swaps to live consensus mode. 
   * Synchronisation leaves us up-to-date at an epoch level, but does not allow us to get up-to-date to a specific round or round phase in consensus. 
 * Timeouts
   * Timeout in Tendermint increases exponentially with round number, in order to give more time for nodes to reach consensus in the presence of delays.
   * $timeoutX(r) = initT timeoutX + r * timeoutDelta;$ 
   * The timeouts are reset for every new height (consensus instance).
* Proposer selection.
  * The proposer is selected each round according to the algorithm [here](https://github.com/tendermint/tendermint/blob/v0.34.x/spec/consensus/proposer-selection.md).
  * You can imagine proposer selection as a round-robin rotation, weighted according to the validator's stake. 
  * The validator set can change at each epoch.


## Diagrams.

A round and its phases:

![consensus_logic](./tendermintround.png)

## Analogues between Nakamoto and Tendermint consensus.

 * Weak subjectivity.
   * In Bitcoin, there are manually embedded block "checkpoints" in the source code, preventing reorgs beyond these points. This is weak subjectivity.
   * In Tendermint, weak subjectivity applies to syncing of the entire chain.
* Syncing algorithm.
  * In both Nakamoto and Tendermint, the node has two consensus modes - one for live consensus and one for offline sync. 
  * When we are not in live consensus, we must skip verifying the timestamps in blocks. This applies in Bitcoin and Tendermint.
  * In Tendermint, sync is simply requesting the linear sequence of blocks up to the current height. In Bitcoin, sync is a greedy iterative search where we continually ask for more blocks from all peers. Although game theoretically, the Bitcoin longest chain will always be the heaviest chain in the limit, we cannot rely on this assumption during sync as an attacker can disconnect us from the nodes with this information. 
  * In Tendermint, we are caught up when we "have at least one peer and its height is at least as high as the max reported peer height". In Bitcoin, we are  caught up when we have a block with the highest accumulated work and each of our peers has no deeper blocks with more accumulated work (we are at the tip).

## Additional readings.

 - https://timroughgarden.github.io/fob21/l/l7.pdf
 - https://arxiv.org/pdf/1807.04938
 - https://jzhao.xyz/thoughts/Tendermint
 - https://groups.csail.mit.edu/tds/papers/Lynch/jacm88.pdf
 - https://tendermint.com/static/docs/tendermint.pdf

## FAQ's.

**How do nodes stay in sync on the value?**

The proposal contains the full value (e.g. block) being proposed.

**What do nodes need to resync?**

In the barebones case, just an online persistent peer to download blocks from.

**How do nodes orient themselves in the current round and timeouts when they first reconnect?**

They download the most recent block, thus **rooting** themselves in the latest block timestamp. (props to [Adi Serendischi](https://x.com/AdiSeredinschi) for this).

**Assuming that validators perform sync by downloading the history from other validators, how do they know when they are "up-to-speed"?**

`IsCaughtUp` method - the node is considered caught up if it has at least one peer and its height is at least as high as the max reported peer height.

**What happens when a node disconnects from and then needs to reconnect to other validators?**

If a node loses network access during live consensus and missed >1 block, it needs to resync first before switching back into live consensus mode.

In the current Tendermint node implementation, this necessitates a manual restart.

**Does the protocol continue when one node is offline? Or there are less than the required number of connections?**

In Paxos consensus, we don't propose unless we have enough acceptors online to constitute a quorum. This is the same in Tendermint.

**How do you prevent an eclipse attack? ie. validators fool you into alternative view of consensus**

Tendermint's trust model involves weak subjectivity.

> See here for the original intro to the concept of "weak subjectivity". Essentially, the first time a node comes online, and any subsequent time a node comes online after being offline for a very long duration (ie. multiple months), that node must find some third-party source to determine the correct head of the chain. This could be their friend, it could be exchanges and block explorer sites, the client developers themselves, or many other actors. PoW does not have this requirement.
> 
> Source: https://vitalik.eth.limo/general/2020/11/06/pos2020.html

**Why do we have to prevote and then precommit? Why not just precommit?**

 * In non-Byzantine environments, consensus requires two communication steps. A node either accepts the proposed value or rejects it. Usually it rejects silently, namely, there is no PREVOTE nil. Once a majority accept a value, the value is decided. This is what the PRECOMMIT messages are for in Tendermint.
 * When we introduce Byzantine behavior, it is possible for the proposer to equivocate. Namely, it can propose different values to different processes. The PREVOTE phase is therefore introduced to reach an agreement on what value the proposer has proposed. If it has equivocated, we don't reach an agreement here. So the PREVOTE phase is intented to agree on the correctness of the proposer.

**Explain why the network needs 3F+1 nodes, what F is, what is the Byzantine fault model?**

In a crash fault, a process simply halts. In a Byzantine fault, it can behave arbitrarily. Crash faults are easier to handle, as no process can lie to another process. Systems which only tolerate crash faults can operate via simple majority rule, and therefore typically tolerate simultaneous failure of up to half of the system. If the number of failures the system can tolerate is f, such systems must have at least 2f + 1 processes.

Byzantine failures are more complicated. In a system of 2f + 1 processes, if f are Byzantine, they can co-ordinate to say arbitrary things to the other f + 1 processes. For instance, suppose we are trying to agree on the value of a single bit, and f = 1, so we have N = 3 processes, A, B, and C, where C is Byzantine. C can tell A that the value is 0 and tell B that itâs 1. If A agrees that its 0, and B agrees that its 1, then they will both think they have a majority and commit, thereby violating the safety condition. Hence, the upper bound on faults tolerated by a Byzantine system is strictly lower than a non-Byzantine one.

