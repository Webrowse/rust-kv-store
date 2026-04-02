use std::collections::HashMap;

use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Parser)]
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

#[derive(Subcommand, Debug, Serialize, Deserialize)]
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
    let mut store: HashMap<String, String> = HashMap::new();

    match kv.command {
        Commands::Set { key, value } => {
            store.insert(key.clone(), value.clone());
            println!("Set: {} = {}", key, value)
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
}
