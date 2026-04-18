# Rust KV Store

## What it is
A key-value store using append-only log persistence.

## Features
- SET / GET / DELETE
- Append-only log (AOF)
- Startup replay
- Log compaction
- Buffered log reading

## Design
- In-memory HashMap
- Log as source of truth
- Replay on startup

## Performance
- ~10k ops replay in ~30ms
- Linear scaling with log size
- Compaction reduces replay cost

## Run
cargo run set key value  
cargo run get key  
cargo run delete key  
cargo run compact  
