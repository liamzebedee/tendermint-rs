# Tendermint

I've been studying recently how to build the Tendermint proof-of-stake protocol. This post collects some learnings.

Consensus
Log replication
State machine replication

Protocol
System of unreliable processes
Building reliability out of unreliability
Protocol discussed as satisfying the properties of finality

Consensus over a single value
Decision value
Consistency
Availability
Partition-tolerance
Terminates
Finality
Agreement


Single shot consensus
The protocol runs in rounds
Each round has a single proposer
If the round fails to commit the value, then it rotates to a different proposer

Nodes engage in these stages of the round
Propose - the proposer broadcasts the value to all nodes
Prevote - the nodes broadcast vote for the block or for nil
Precommit - the nodes broadcast a commit for the block or nil

Each stage is subject to timeouts
FLP impossibility result - it is impossible in a fully asynchronous message-passing distributed system, in which at least one process may have a crash failure, for a deterministic algorithm to achieve consensus

So:
- node proposes
- all other nodes await proposal or wait for timeout
- nodes broadcast a vote on the value they've seen, or the most recent value they've locked on, or nil
- nodes await all other votes or timeout
- they then compute the decision value. the decision value is set when a node sees 2f+1 prevotes for it
- the node then broadcasts a precommit for the decision value, or nil
- if the node sees 2f+1 precommits for the proposal, then that value is agreed. 





In a crash fault, a process simply halts. In a Byzantine fault, it can behave arbitrarily. Crash faults are easier to handle, as no process can lie to another process. Systems which only tolerate crash faults can operate via simple majority rule, and therefore typically tolerate simultaneous failure of up to half of the system. If the number of failures the system can tolerate is f, such systems must have at least 2f + 1 processes

Byzantine failures are more complicated. In a system of 2f + 1 processes, if f are Byzantine, they can co-ordinate to say arbitrary things to the other f + 1 processes. For instance, suppose we are trying to agree on the value of a single bit, and f = 1, so we have N = 3 processes, A, B, and C, where C is Byzantine, as in Figure 2.2. C can tell A that the value is 0 and tell B that it’s 1. If A agrees that its 0, and B agrees that its 1, then they will both think they have a majority and commit, thereby violating the safety condition. Hence, the upper bound on faults tolerated by a Byzantine system is strictly lower than a non-Byzantine one.

In fact, it can be shown that the upper limit on f for Byzantine faults is f < N/3 [78]. Thus, to tolerate a single Byzantine process, we require at least N = 4. Then the faulty process can’t split the vote the way it was able to when N = 3.


Byzantine fault tolerance
2 phase commit - prepare and commit



