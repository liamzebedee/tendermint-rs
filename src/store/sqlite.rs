use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

// Schema definition
table! {
    prevotes (id) {
        id -> Integer,
        height -> Integer,
        round -> Integer,
        validator -> Text,
        block_hash -> Text,
        timestamp -> Text,
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = prevotes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Prevote {
    pub id: i32,
    pub height: i32,
    pub round: i32,
    pub validator: String,
    pub block_hash: String,
    pub timestamp: String,
}

pub struct SqliteStore {
    pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

impl SqliteStore {
    pub fn new() -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        // Create tables
        let mut conn = pool.get().unwrap();
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS prevotes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                height INTEGER NOT NULL,
                round INTEGER NOT NULL,
                validator TEXT NOT NULL,
                block_hash TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
        )
        .execute(&mut conn)
        .expect("Failed to create table");

        SqliteStore {
            pool: Arc::new(pool),
        }
    }

    pub fn insert_prevote(&self, height_: i32, round_: i32, validator_: &str, block_hash_: &str) {
        use self::prevotes::dsl::*;
        let mut conn = self.pool.get().unwrap();
        
        let new_prevote = Prevote {
            id: 0, // This will be auto-incremented
            height: height_,
            round: round_,
            validator: validator_.to_string(),
            block_hash: block_hash_.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        diesel::insert_into(prevotes)
            .values(&new_prevote)
            .execute(&mut conn)
            .expect("Failed to insert prevote");
    }

    pub fn get_prevotes(&self) -> Vec<Prevote> {
        use self::prevotes::dsl::*;
        let mut conn = self.pool.get().unwrap();
        
        prevotes
            .order_by(height.asc())
            .then_order_by(round.asc())
            .load::<Prevote>(&mut conn)
            .expect("Failed to load prevotes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prevotes() {
        let store = SqliteStore::new();
        
        // Insert 5 prevotes
        store.insert_prevote(1, 0, "validator1", "hash1");
        store.insert_prevote(1, 1, "validator2", "hash2");
        store.insert_prevote(2, 0, "validator3", "hash3");
        store.insert_prevote(2, 1, "validator4", "hash4");
        store.insert_prevote(3, 0, "validator5", "hash5");

        // Get and print all prevotes
        let prevotes = store.get_prevotes();
        for prevote in prevotes {
            println!(
                "Height: {}, Round: {}, Validator: {}, Block Hash: {}",
                prevote.height, prevote.round, prevote.validator, prevote.block_hash
            );
        }
    }
} 