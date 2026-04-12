use std::fs::OpenOptions;
use std::io::Write;
use std::{collections::HashMap, fs, path::Path};

use clap::{Parser, Subcommand};

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
    let mut store: HashMap<String, String> = HashMap::new();
    if Path::new("log.db").exists() {
        let content = fs::read_to_string("log.db").unwrap_or_default();

        for line in content.lines() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "SET" => {
                    if parts.len() >= 3 {
                        let key = parts[1].to_string();
                        let value = parts[2..].join(" ");
                        store.insert(key, value);
                    }
                }

                "DEL" => {
                    if parts.len() == 2 {
                        let key = parts[1];
                        store.remove(key);
                    }
                }

                _ => {}
            }
        }
    }

    match kv.command {
        Commands::Set { key, value } => {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.db")
                .expect("failed to open log.db");
            writeln!(file, "SET {} {}", key, value).unwrap();
            store.insert(key, value);
        }
        Commands::Get { key } => match store.get(&key) {
            Some(value) => println!("for key: {}, value : {}", key, value),
            None => println!("The value for key : {}, is not found", key),
        },
        Commands::Delete { key } => {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.db")
                .expect("failed to open log.db");
            store.remove(&key);
            writeln!(file, "DEL {}", key).unwrap();
        }
        Commands::Compact => {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("log.db")
                .expect("failed to compact log.db");

            for (key, value) in &store {
                writeln!(file, "SET {} {}", key, value).expect("write failed");
            }
            println!("logs compacted");
        }
    }
}
