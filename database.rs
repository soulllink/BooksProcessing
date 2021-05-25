use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
struct Database {
    conn: Connection,
    dbname: String,
}

impl Database {
    fn new(name: String) -> Database {
        Database {
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
    fn insert(&self) {
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
    fn transaction(&self, trxs: Vec) {
        let tx = &self.conn.transaction;
            
            for s, fq in trxs {
                tx.execute(format!(
                        "INSERT INTO {name} (words, freq) VALUES (?1, ?2)", 
                        name = &self.dbname.replace(".", "")
                        )
                        .as_str(),
                        &[&s, &fq.to_string()],
                    )
                    
                , NO_PARAMS);
            }

        tx.commit();
    }
}

fn collector(name: String, word: String, freq: u32) {
    
}
