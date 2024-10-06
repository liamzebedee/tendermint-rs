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
 - sync: rewrite algo so that time is abstracted away, and we can simulate old consensus rounds.

Demo network:

 - [x] startup node.
 - [x] try listen for other nodes.
 - [x] peer configs, network configs.
 - [ ] peer handshake - dial and allow peers to add each other to the routing table. connect with "senders" channel.
 - [ ] connect to server listing all validators and their ip's.
 - [ ] choose get value function - idk probably most recently 

Ideas:
 - [ ] motomint - tendermint but the proposer set is dictated by POW. Basically 

