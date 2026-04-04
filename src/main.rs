use std::{collections::HashMap, fs, path::Path};

use clap::{Parser, Subcommand};
use serde_json;

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
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Delete{
        key: String,
    },
}
    
fn main () {
    let kv = Kv::parse();
    let mut store: HashMap<String, String> = if Path::new("store.json").exists() {
        let contents = fs::read_to_string("store.json").unwrap_or_default();
        serde_json::from_str(&contents).unwrap()
        
    } else {
        HashMap::new()
    };

    match kv.command {
        Commands::Set { key, value } => {
            store.insert(key.clone(), value.clone());
            println!("Set: {} = {}", key, value);


        },
        Commands::Get { key } => {
            match store.get(&key) {
                Some(value) => println!("for key: {}, value : {}", key, value),
                None => println!("The value for key : {}, is not found", key),
            }
        },
        Commands::Delete { key } => {
            store.remove(&key);
            println!("Deleted : {}", key);

        }
    }
    let json = serde_json::to_string(&store).unwrap_or_default();
    fs::write("store.json", json).unwrap();
}
