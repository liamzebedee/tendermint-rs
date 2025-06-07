use crate::types::{Transaction, Vote};

mod rocks;
pub use rocks::RocksStore;

const PREFIX_TRANSACTION: str = "transaction:";
const PREFIX_PREVOTE: str = "prevote:";
const PREFIX_PRECOMMIT: str = "precommit:";

fn key_for_transaction(tx: Transaction) -> Vec<u8> {
    let mut key = PREFIX_TRANSACTION.as_bytes().to_vec();
    key.extend(tx.hash());
    key
}

fn key_for_prevote(prevote: Vote) -> Vec<u8> {
    let mut key = PREFIX_PREVOTE.as_bytes().to_vec();
    key.extend(&prevote.inner.height.to_le_bytes());
    key
}

fn key_for_precommit(precommit: Vote) -> Vec<u8> {
    let mut key = PREFIX_PRECOMMIT.as_bytes().to_vec();
    key.extend(&precommit.inner.height.to_le_bytes());
    key
}