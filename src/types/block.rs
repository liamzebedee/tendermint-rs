use crate::protos::Block as ProtoBlock;
use crate::types::Transaction;
use sha3::{Sha3_256, Digest};

// TODO: fix transactions so no repetition here.

pub struct Block {
    pub inner: ProtoBlock,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, proposer: Vec<u8>, prev_block_hash: Vec<u8>, prev_block_height: u64) -> Self {
        let block = ProtoBlock {
            proposer,
            previous_block_hash: prev_block_hash,
            height: prev_block_height + 1,
            txs: Vec::new(),
        };

        Self {
            inner: block,
            transactions,
        }
    }

    pub fn from_message(msg: ProtoBlock) -> Self {
        let transactions = msg.txs.iter().cloned().map(Transaction::from_message).collect();
        Self {
            inner: msg,
            transactions,
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.inner.proposer);
        hasher.update(&self.inner.previous_block_hash);
        for tx in &self.transactions {
            hasher.update(tx.hash());
        }
        hasher.finalize().to_vec()
    }

    pub fn validate(&self) -> bool {
        // TODO: implement block validation
        // This should validate:
        // 1. All transactions are valid
        // 2. Block hash is correct
        // 3. Any other block-specific validation rules
        true
    }
} 