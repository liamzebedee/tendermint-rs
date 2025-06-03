config
    validators
        ip
        pubkey

getvalue
    get current value at round

message wrappers
    for each protocol buffers message
    we want to wrap it in our own datatype

    transaction
        .hash function which computes the sha256 hash
        .verify_sig function which validates the signature is correct
    
    vote
        .verify_sig function which validates the signature is correct for the sender
    
    block
        .hash function which computes canonical hash for list of txs
        stores block in store by hash for later pickup

AbstractMonotonicClock
    used for local testing of timeouts etc.
    simulates a monotonic clock
    has a timeout method used in tendermint algo

    methods
        crank(time: ms)
        get_time(time: ms)
        async timeout(time: ms)
            fires after time

validator
    backing store
        trait
        uses leveldb on backend
        methods:
            get_prevotes()
            get_precommits()
            insert_prevote()
            insert_precommit()

    startup
        ask all peers
        download latest block
        "root" in this block timestamp
        now node is oriented

    consensus step
        propose
        prevote
        precommit
        decision -> invoke decision handler

test
    factory
        generates 5 validators
            0.0.0.0:port % i
            pubkey
            leveldb database name -> use purely in-memory version for tests
