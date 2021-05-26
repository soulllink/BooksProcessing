use regex::Regex;
use rusqlite::{Connection, Result, Transaction, NO_PARAMS};
use std::char::ToLowercase;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::string::ToString;

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
        &self
            .conn
            .execute(
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
    fn transaction(&self, trxs: HashMap<String, u32>) {
        let tx = &self.conn.transaction();
        for (s, fq) in trxs.iter() {
            tx.execute(
                format!(
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

fn collector(trxs: HashMap<String, u32>, word: String, freq: u32) {
    trxs.insert(word, freq)
}

fn sender(mut trxs: HashMap<String, u32>, consql: DatabaseSQL) {
    if assert!(trxs.capacity() >= 10) {
        consql.transaction(&trxs);
        trxs.clear();
    };
}

fn main() {
    //IOinput
    println!("Filename?");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    //filter \n and other garbageinput
    let re = Regex::new(r"([0-9a-zA-Z\._-]+.(txt))").unwrap();
    filename = re.find(&filename).unwrap().as_str().to_string();
    let dataname = filename.clone();
    //verify input
    println!("|{}|", filename.clone());
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    //Database init
    let DataSQL = DatabaseSQL::new(&dataname);
    DataSQL.exec();
    let mut trxs: HashMap<String, u32> = HashMap::new();

    //
    // startmapping
    //
    let mut frequency: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        //replace for sep.marks
        &line
            .to_lowercase()
            .replace(",", " ,")
            .replace(":", " :")
            .replace(";", " ;")
            .replace("!", " !")
            .replace("(", " (")
            .replace(")", " )")
            .replace("…", " …")
            .replace("?", " ?")
            .replace("[", " [")
            .replace("]", " ]")
            .replace("«", " «")
            .replace("»", " »");

        let out: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        for word in out {
            *frequency.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    println!("{:?}", frequency);

    for (s, fq) in frequency {
        //insert(&mut conn, &dataname, s, fq);
        collector(&trxs, s, fq);
        sender(&trxs, &DataSQL);
    }
}
