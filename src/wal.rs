use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter, Write, BufRead};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogEntry {
    SET {
        key: String,
        value: String,
        expires_in: Option<u64>,
    },
    DEL {
        key: String,
    },
}

pub struct WAL {
    file: BufWriter<File>,
    path: String,
}


impl WAL {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .expect("Failed to open WAL file");

        WAL {
            file: BufWriter::new(file),
            path: path.to_string(),
        }
    }

    pub fn append_log(&mut self, entry: LogEntry) {
      let line = serde_json::to_string(&entry).expect("Failed to serialize log entry");
        writeln!(self.file, "{}", line).expect("Failed to write to WAL file");
        self.file.flush().expect("Failed to flush WAL file");
    }

    pub fn load_log(&mut self) -> Vec<LogEntry> {
        let file = File::open(&self.path).expect("Failed to open WAL file");
        let reader = BufReader::new(file);

        reader.lines()
            .filter_map(|line| {
                line.ok().and_then(|l| serde_json::from_str::<LogEntry>(&l).ok())
            })
            .collect()
    }
}