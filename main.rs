use regex::Regex;
use rusqlite::{Connection, NO_PARAMS};
use std::char::ToLowercase;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::string::ToString;

//import
mod Database;

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
    let DataSQL = Database::new(&dataname);
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
    }
}
