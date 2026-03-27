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
    set,
    get,
    delete,
}
    
fn main () {
    let kv = Kv::parse();
    match kv.command {
        Commands::set => {},
        Commands::get => {},
        Commands::delete => {},
    }

}
