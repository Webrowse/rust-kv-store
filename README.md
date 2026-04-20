# Rust KV Store

## Overview  

A minimal key-value store implemented in Rust using an append-only log for persistence.

Designed to explore durability, log replay, and trade-offs between simplicity and performance without external dependencies.

---

## Features  
- SET / GET / DELETE operations
- Append-only log (AOF)
- Full state reconstruction via replay
- Log compaction
- Buffered file reads

---

## How It Works

#### 1. On startup:
   - Read log file sequentially
   - Replay all operations into an in-memory HashMap   


#### 2. On write:
   - Append operation to log file
   - Sync to disk
   - Update in-memory state

#### 3. On read:
   - Serve directly from HashMap (O(1))

---

## Design Decisions  

#### In-memory HashMap  
  - Enables fast O(1) reads  
  - Trade-off: entire dataset must fit in memory  
  
#### Append-only log 
  - Simplifies persistence model
  - Avoids in-place file mutation
  - Trade-off: file grows indefinitely without compaction

#### Log as source of truth
  - Ensures recoverability after crash
  - In-memory state is always reconstructible

#### sync_all for durability
  - Guarantees OS-level persistence
  - Trade-off: slower writes due to disk I/O

---

## Performance Characteristics

- Reads: O(1)
- Writes: O(1) append + disk sync
- Startup: O(n) log replay

Example:
- ~10k operations replayed in ~30ms  
  (Measured on local machine, SSD, release build)

Primary bottleneck:
- Disk I/O during sync_all
- Replay time grows linearly with log size

---

## Limitations

- Log file grows without bounds until compaction
- No concurrency support (single-process only)
- Entire dataset loaded into memory
- No crash recovery during partial writes
- No indexing or partial replay

---

## Future Improvements

- Incremental or background compaction
- Async I/O for better throughput
- File locking for multi-process safety
- Snapshotting to reduce replay cost
- Checksums to detect log corruption

---

## Run

```markdown
cargo run set key value
cargo run get key
cargo run delete key
cargo run compact
```
