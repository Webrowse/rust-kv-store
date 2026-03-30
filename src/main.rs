use clap::{Parser, Subcommand};

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

#[derive(Subcommand)]
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
    match kv.command {
        Commands::Set { key, value } => println!("Set: {} = {}", key, value),
        Commands::Get { key } => println!("Get: {}", key),
        Commands::Delete { key } => println!("Delete: {}", key),
    }

}
