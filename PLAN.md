# Implementation plan.

Features:

 - [x] `run_round` runs a single round.
 - [x] encapsulate round state, so we can pass it into `run_round` and repeat.
 - [ ] test `run_round` independently.
 - [x] add loop which runs rounds.
 - [x] hooks/extensions/events api: 
   - [x] get value
   - [x] on new decision
 - [ ] fix consensus height + stuff. commit data to log on disk.
 - [ ] add pubkey identities for nodes. add signatures to node messages.
 - [ ] add gRPC for data types.
 - [ ] implement dynamic timeouts to allow network to resolve with backoff.
 - [ ] change node to start up on a network interface and listen to messages.
 - [ ] add node sync so it restarts and gets history from other nodes for height before it.
 - [ ] check precommits/prevotes are unique.

Ideas:
 - [ ] 

