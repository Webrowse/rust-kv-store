use std::time::Instant;
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
    let start = Instant::now();
    let (store, count) = load_store(); 
    println!("Total time taken for HashMap: {:?}", start.elapsed());
    println!("Total ops cycles: {}", count);

    match kv.command {
        Commands::Set { key, value } => {
            append_set(&key, &value);
            println!("Set {} = {}", key, value);
        }
        Commands::Get { key } => match store.get(&key) {
            Some(value) => println!(r#""{}" = "{}""#, key, value),
            None => println!(r#""{}" is not found"#, key),
        },
        Commands::Delete { key } => {
            append_del(&key);
            println!("DEL {}", key);
       },
        Commands::Compact => {
            compact(&store);
           println!("logs compacted");
        }
    }
}
