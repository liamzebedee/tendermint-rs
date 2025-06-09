config
    validators
        ip
        pubkey

getvalue
    get current value at round

types
    for each protocol buffers message
    we want to wrap it in our own datatype

    proposal
        new(keypair, Block, round, height, clock) -> signed message

        from_message
            wraps the message as an inner type
            parses block

        sig_envelope
            all fields in proto message

        sig_verify

        validate(clock)
            validates with an abstract clock which validates the time is recent (within idk 30s)
            verifies the sig

    vote
        new(keypair, VoteMessage) -> signed message

        from_message
            wraps the message as an inner type
        
        get_type -> enum{Precommit, Prevote}

        sig_envelope
            all fields in proto message

        sig_verify

        validate(clock)
            validates with an abstract clock which validates the time is recent (within idk 30s)
            verifies the sig

    transaction
        new(keypair, data) -> signed message

        from_message
            wraps the message as an inner type
        
        .hash function which computes the sha256 hash
        
        sig_verify
        
        sig_envelope
            all fields in proto message
        
        validate()
            validate it works according to state machine??
    
    Block
        new(txs: Vec<Transaction>, proposer, prev_block_hash)
        from_message

        hash()
            H(proposer, prevblockhash, txs)
        
        validate


txs_to_proposal(txs, previous_block_hash, height, proposer_keypair, round) -> Block, ProposeMessage
    takes txs, makes block
    makes hash
    this is value for proposal
    signs proposal message


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
    get_status() 
        Syncing(synced_height: 00, latest_height: 00, missing: 00)
        Live

    backing store
        trait
        uses leveldb on backend
        tables:
            precommits
            prevotes
            blocks
            transactions
            peers
        serialises data to the leveldb format (set of columns) using serde
        methods:
            get_prevotes()
            get_precommits()
            insert_prevote()
            insert_precommit()
            get_sync_tip()
                load 100 precommits per chunk from store height descending until we have a majority that sign `height`
                this is our tip

    full_sync(height)
        call GetHistory for blocks until store.get_sync_tip

    startup
        ask all peers GetLatest
            wait for responses from at least MAJORITY peers
            if we cannot reach majority, loop in 1s timeouts
            print "sync: waiting for connected to peer majority"
        ingest block
            if not seen parent
            get this block's parent
        "root" in this block timestamp
        sync from this block backwards until done

    consensus round (abstract_clock, peers)
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


