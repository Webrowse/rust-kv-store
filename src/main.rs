// use std::fs::OpenOptions;
// use std::io::Write;
// use std::collections::HashMap;
use std::time::Instant;
// use std::io::{BufRead, BufReader};
// use std::fs::File;
use clap::{Parser, Subcommand};

use crate::{log::{append_del, append_set, compact}, store::load_store};

mod store;
mod log ;

#[derive(Parser, Debug)]
#[command(
    name = "KVStore", 
    version = "0.1.0", 
    about = "KV Store", 
    long_about = None
    )]
struct Kv {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Compact,
}

fn main() {
    let kv = Kv::parse();
    // let mut ops = 0;
    // let mut store: HashMap<String, String> = HashMap::new();
    let start = Instant::now();
    // if Path::new("log.db").exists() {
    //     let file = File::open("log.db").unwrap();
    //     let reader = BufReader::new(file);
    //
    //     for line in reader.lines() {
    //         ops += 1;
    //         let line = line.unwrap();
    //         let parts = line.split_whitespace().collect::<Vec<&str>>();
    //         if parts.is_empty() {
    //             continue;
    //         }
    //
    //         match parts[0] {
    //             "SET" => {
    //                 if parts.len() >= 3 {
    //                     let key = parts[1].to_string();
    //                     let value = parts[2..].join(" ");
    //                     store.insert(key, value);
    //                 }
    //             }
    //
    //             "DEL" => {
    //                 if parts.len() == 2 {
    //                     let key = parts[1];
    //                     store.remove(key);
    //                 }
    //             }
    //
    //             _ => {}
    //         }
    //     }
    // }
    let  store = load_store(); 
    println!("Total time taken for HashMap: {:?}", start.elapsed());
    // println!("Total number of operations: {}", ops);

    match kv.command {
        Commands::Set { key, value } => {
            append_set(&key, &value);
            // let mut file = OpenOptions::new()
            //     .create(true)
            //     .append(true)
            //     .open("log.db")
            //     .expect("failed to open log.db");
            // writeln!(file, "SET {} {}", key, value).expect("write failed");
            // store.insert(key, value);
            // file.sync_all().expect("flush failed");
        }
        Commands::Get { key } => match store.get(&key) {
            Some(value) => println!(r#""{}" = "{}""#, key, value),
            None => println!(r#""{}" is not found"#, key),
        },
        Commands::Delete { key } => {
            append_del(&key);
            // let mut file = OpenOptions::new()
            //     .create(true)
            //     .append(true)
            //     .open("log.db")
            //     .expect("failed to open log.db");
            // store.remove(&key);
            // writeln!(file, "DEL {}", key).expect("write failed");
            // file.sync_all().expect("flush failed");
        },
        Commands::Compact => {
            compact(&store);
            // let mut file = OpenOptions::new()
            //     .create(true)
            //     .write(true)
            //     .truncate(true)
            //     .open("log.db")
            //     .expect("failed to compact log.db");
            //
            // for (key, value) in &store {
            //     writeln!(file, "SET {} {}", key, value).expect("write failed");
            // }
            println!("logs compacted");
            // file.sync_all().expect("flush failed");
        }
    }
    println!("Total time taken for whole program: {:?}", start.elapsed());
}
