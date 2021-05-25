use rusqlite::{Connection, NO_PARAMS};
use std::collections::HashMap;

#[derive(Debug)]
struct DatabaseSQL {
    conn: Connection,
    dbname: String,
}

impl DatabaseSQL {
    fn new(name: String) -> DatabaseSQL {
        DatabaseSQL {
            conn: Connection::open("data.db").unwrap(),
            dbname: name,
        }
    }
    fn exec(&self) {
        &self.conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {name} (
            id INTEGER PRIMARY KEY,
            words TEXT,
            freq INTEGER
          )",
                name = &self.dbname.clone().replace(".", "")
            )
            .as_str(),
            NO_PARAMS,
        )
        .unwrap()
    }
    fn insert(&self, word: String, freq: u32) {
        &self
            .conn
            .execute(
                format!(
                    "INSERT INTO {name} (words, freq) VALUES (?1, ?2)",
                    name = &self.dbname.replace(".", "")
                )
                .as_str(),
                &[&word, &freq.to_string()],
            )
            .unwrap()
    }
    fn transaction(&self, trxs: HashMap) {
        let tx = &self.conn.transaction;
            
        for (s, fq) in trxs.iter() {
            tx.execute(format!(
                    "INSERT INTO {name} (words, freq) VALUES (?1, ?2)", 
                    name = &self.dbname.replace(".", "")
                    )
                    .as_str(),
                    &[&s, &fq.to_string()],
            );
        }

        tx.commit();
    }
}

fn collector(trxs: HashMap, word: String, freq: u32) {
    *trxs.insert( word, freq )
}

fn sender(trxs: HashMap, consql: DatabaseSQL) {
    if *trxs.capacity() >= 10 {
        *consql.transaction(&trxs);
        *trxs.clear();
    }
}


