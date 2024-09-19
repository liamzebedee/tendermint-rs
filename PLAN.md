# Implementation plan.

Features:

 - [x] `run_round` runs a single round.
 - [x] encapsulate round state, so we can pass it into `run_round` and repeat.
 - [ ] test `run_round` independently.
 - [x] add http node api's
 - [x] add loop which runs rounds.
 - [x] hooks/extensions/events api: 
   - [x] get value
   - [x] on new decision
 - [x] add pubkey identities for nodes. add signatures to node messages.
 - [ ] fix consensus height + stuff. commit data to log on disk.
 - [ ] implement dynamic timeouts to allow network to resolve with backoff.
 - [x] change node to start up on a network interface and listen to messages.
 - [ ] add node sync so it restarts and gets history from other nodes for height before it.
 - [ ] check precommits/prevotes are unique.

Demo network:

 - [ ] startup node.
 - [ ] connect to server listing all validators and their ip's.
 - [ ] try listen for other nodes.
 - [ ] choose get value function - idk probably most recently 

Ideas:
 - [ ] motomint - tendermint but the proposer set is dictated by POW. Basically 

