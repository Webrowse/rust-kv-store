use std::fs::OpenOptions;
use std::io::Write;

pub fn append_set(key: &str, value: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.db")
        .expect("failed to open log.db");
    writeln!(file, "SET {} {}",key, value).expect("write failed");
    file.sync_all().expect("sync failed");
}

pub fn append_del(key: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.db")
        .expect("failed to open log.db");
    writeln!(file, "DEL {}", key).expect("write failed");
    file.sync_all().expect("sync failed");
}

pub fn compact(store: &std::collections::HashMap<String, String>){
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("log.db")
        .expect("failed to open log.db");
    
    for (k, v) in store {
        writeln!(file, "SET {} {}", k, v).expect("fail to write");
    }
    file.sync_all().expect("sync failed");
}
