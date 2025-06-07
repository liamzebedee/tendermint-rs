use rocksdb::{DB, Options, ColumnFamilyDescriptor};
use std::sync::Arc;
use crate::types::Vote;
use serde::{Serialize, Deserialize};

const CF_PREVOTES: &str = "prevotes";

#[derive(Debug, Serialize, Deserialize)]
struct PrevoteRecord {
    height: u64,
    validator: String,
    timestamp: u64,
}

pub struct RocksStore {
    db: Arc<DB>,
}

impl RocksStore {
    pub fn new_in_memory() -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cf_opts = Options::default();
        let cf_descriptors = vec![ColumnFamilyDescriptor::new(CF_PREVOTES, cf_opts)];

        let db = DB::open_cf_descriptors(&opts, ":memory:", cf_descriptors)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn put_prevote(&self, vote: &Vote) -> Result<(), rocksdb::Error> {
        let cf = self.db.cf_handle(CF_PREVOTES).unwrap();
        
        let record = PrevoteRecord {
            height: vote.inner.height,
            validator: vote.inner.validator.clone(),
            timestamp: vote.inner.timestamp,
        };

        let key = vote.inner.height.to_le_bytes();
        let value = bincode::serialize(&record).unwrap();
        
        self.db.put_cf(&cf, key, value)
    }

    pub fn get_prevotes(&self) -> Result<Vec<PrevoteRecord>, rocksdb::Error> {
        let cf = self.db.cf_handle(CF_PREVOTES).unwrap();
        let mut prevotes = Vec::new();
        
        let iter = self.db.iterator_cf(&cf, rocksdb::IteratorMode::Start);
        for item in iter {
            let (_, value) = item?;
            let record: PrevoteRecord = bincode::deserialize(&value).unwrap();
            prevotes.push(record);
        }
        
        Ok(prevotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Vote, VoteInner};

    #[test]
    fn test_prevotes_storage() {
        let store = RocksStore::new_in_memory().unwrap();
        
        // Create 5 test prevotes
        for i in 1..=5 {
            let vote = Vote {
                inner: VoteInner {
                    height: i,
                    validator: format!("validator_{}", i),
                    timestamp: i * 1000,
                    ..Default::default()
                },
            };
            
            store.put_prevote(&vote).unwrap();
        }

        // Retrieve and verify prevotes
        let prevotes = store.get_prevotes().unwrap();
        assert_eq!(prevotes.len(), 5);

        // Print prevotes in order
        println!("\nStored prevotes:");
        for prevote in prevotes {
            println!("Height: {}, Validator: {}, Timestamp: {}", 
                prevote.height, 
                prevote.validator, 
                prevote.timestamp
            );
        }
    }
} 