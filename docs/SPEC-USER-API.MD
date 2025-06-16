
examples/blockchain.rs
    type Transaction {
        from;
        to;
        value;
    }

    type State {
        HashMap balances Publickey -> u64 amount;
    }

    initial state = State {
        built-in public key -> 100M
    }

    type StateMachine {
        apply(tx: Transaction, state: State) {
            if tx.height < MAX_INT {
                v1 txs
            }
        }

        v1_apply() {
            validate signature
            update from balance
            update to balance
            defauts 0, read from sate
        }
    }

    struct Blockchain {
        mempool: Vec<Transaction>
        state: State
    }

    impl Blockchain {
        fn ingest_tx(tx) {
            add to mempool
            gossip tx
        }
    }

    impl StateMachine for Blockchain {
        receive_gossip(txs) {

        }

        // This is called serially by the ConsensusEngine for new blocks from 0..latest_height.
        // It gives the engine the ability to process new txs.
        // It must not error or we're screwed.
        transition(blocks) {
            state2 = current_state.clone()
            for block in blocks {
                fmt!("procesisng block height=xx txs=xx")
                for tx in block {
                    fmt!("processing tx block_height=xx txindex=xx")
                    state2 = StateMachine.apply(tx, state2)
                }
            }
            current_state = state2
        }

        get_next_value() {
            takes current tx set from mempool
            state2 = current_state.clone()
            for tx in mempool {
                state2 = StateMachine.apply(tx, state2)
            }
            block = Block {
                txs,
            }
            return block
        }
    }

    trait ConsensusConsumer {
        receive_gossip(tx)
        get_next_value(height) -> bytes
    }

    // API for consumption.
    // 
    //
    struct TendermintConsensus {
        new(consumer: ConsensusConsumer)
        connect_validators(validators)
    }

    main {
        validators = vec![] // generate 5 with keypairs
        validator_config_log = vec![
            ValidatorsConfig {
                start_height: 0,
                vec! [
                    Validator {
                        pubkey
                    },
                    Validator {
                        pubkey
                    },
                    Validator {
                        pubkey
                    },
                    Validator {
                        pubkey
                    },
                    Validator {
                        pubkey
                    }
                ]
            },
        ]

        for each validator:
            state = genesis_state
            machine = Blockchain(genesis_state)
            tendermint = TendermintConsensus(machine)
            validator = tendermint.get_validator(pubkey)
            validator.connect_validators(...)
            validator.start()
    }



## Bigtable

key things
    master node does allocation
    state = 
        tablets
            tablet {
                start_key
                end_key
                size
            }
        assignments
            tablet -> tabletserver
        tabletservers

    control loop
        receive status <- tabletservers
        update tablet size

        foreach tablet
            if tablet.size >= 0.7*MAX_TABLET_CAPACITY {
                lock tablet

                split tablet:
                tabletL = {
                    start_key = tablet.start_key
                    end_key = tablet.end_key / 2
                    status: LOCKED
                }
                tabletR = {
                    start_key = tablet.end_key / 2
                    end_key = tablet.end_key
                    status: LOCKED
                }
                assign tabletL to tabletserver_tablet
                assign tabletR to tabletserver_tabletR

                emit
                    Lock(tablet)
                    Creation(tabletL, info)
                    Creation(tabletR, info)
                    Assignment(tabletL, tabletserver_tablet)
                    Assignment(tabletR, tabletserver_tabletR)
            }
    
        foreach tabletserver
            last_heartbeat = heartbeats[tabletserver]
            if SERVER_LIVE_PERIOD < (now() - last_heartbeat):
                // tabletserver is dead
                tablet.lock()
                new_tabletserver = choose_new_tabletserver()
                assignments[tablet] = new_tabletserver

                emit
                    Lock(tablet)
                    Died(tabletserver)
                    Assignment(tablet, new_tabletserver)
        
        on
            { assignments[tablet] live }
                unlock tablet
                emit
                    Unlock(tablet)
            { tabletserver_tabletL live }
                unlock tabletL
                emit
                    Unlock(tabletL)
            { tabletserver_tabletR live }
                unlock tabletR
                emit
                    Unlock(tabletR)

How to ensure the tabletservers actually commit their reads/writes? 
ie. in a byzantine sense
they could fail
and then where do the writes persist? 
really we just need to persist the writes until it's flushed to gfs and then it's fine
writes are persisted to a gfs WAL
that's write (lol)
so that's fine? 
because GFS just relies on chunkservers
so that's n=3 writes 
that's not too bad.


