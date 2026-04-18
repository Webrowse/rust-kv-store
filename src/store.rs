use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_store() -> (HashMap<String, String>, usize){
    let mut store = HashMap::new();
    let mut ops = 0;

    if Path::new("log.db").exists() {
        let file = File::open("log.db").expect("Fail to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line: String = line.expect("fail to read line");
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty(){
                continue;
            }

            ops += 1;
            match parts[0] {
                "SET" => {
                    if parts.len() >= 3 {
                        let key = parts[1].to_string();
                        let value = parts[2..].join(" ");
                        store.insert(key, value);
                    }
                },
                "DEL" => {
                    if parts.len() == 2 {
                        store.remove(parts[1]);
                    }
                },
                _ => {},
            }
        }
    }
    (store, ops)
}
