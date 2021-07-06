use regex::Regex;
use rusqlite::*;
//use rusqlite::Connection::{execute, transaction};
use std::char::ToLowercase;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::string::ToString;

fn main() {
    //init
    println!("Filename?");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    let re = Regex::new(r"([0-9a-zA-Z\._-]+.(txt))").unwrap();
    filename = re.find(&filename).unwrap().as_str().to_string();
    println!("|{}|", &filename);
    //openfile
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    //mapping
    let mut frequency: HashMap<String, u32> = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        //replace for sep.marks
        let tmpstr = line
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

        let out: Vec<&str> = tmpstr.split(" ").collect::<Vec<&str>>();

        for word in out {
            *frequency.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    //SQL save
    let mut conn = Connection::open("data.db").expect("Err");
    dbinit(&mut conn,&filename);
    dbpush(&mut conn,&filename, frequency);

    // for (s, fq) in frequency {
            
    // }



}

fn dbinit(db: &mut rusqlite::Connection, dbname: &String) {
    let querry = format!("CREATE TABLE IF NOT EXISTS {name} (id INTEGER PRIMARY KEY, words TEXT, freq INTEGER)", name = dbname.replace(".", ""));
    db.execute(querry.as_str(), [],).unwrap();
}

fn dbpush(db: &mut Connection, dbname: &String, freq: HashMap<String, u32>) {
    let tx = db.transaction().expect("Err");
    let querry = format!("INSERT INTO {name} (words, freq) VALUES (?1, ?2)", name = dbname.replace(".", ""));
    for (s, fq) in freq {
        tx.execute(&querry.as_str(), &[&s, &fq.to_string()],);
    }
    tx.commit();

}