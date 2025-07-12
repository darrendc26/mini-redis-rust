use tokio;
use tokio::net::TcpListener;
use tokio::{io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};
use tokio::net::TcpStream;
use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::parser::*;
use crate::db::*;
use crate::wal::*;

pub async fn run_server(addr: &str, db: Arc<Mutex<DB>>, wal: Arc<Mutex<WAL>>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);
    
    
    let entries = wal.lock().unwrap().load_log();
    for entry in entries {
        match entry {
            LogEntry::SET { key, value, expires_in } => {
                let mut db = db.lock().unwrap();
                db.set(key.clone(), value.clone(), expires_in);
                println!("Restored SET entry: key={}, value={}, expires_in={:?}", key, value, expires_in);
            }
            LogEntry::DEL { key } => {
                let mut db = db.lock().unwrap();
                db.del(&key);
                println!("Restored DEL entry for key: {}", key);
            }
        }
    }

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        let db = db.clone();
        let wal = wal.clone();
        // Spawn a new task per connection
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, db, wal).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}


async fn handle_connection(socket: TcpStream, db: Arc<Mutex<DB>>, wal: Arc<Mutex<WAL>>) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.into_split();

    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        match parse(&line){
            Ok(Command::SET { key, value, expires_in }) => {
                {
                    let mut db = db.lock().unwrap();
                    db.set(key.clone(), value.clone(), expires_in);
                } // db lock is dropped here
                
                let log_entry = LogEntry::SET {
                    key: key.clone(),
                    value: value,
                    expires_in,
                };
                {
                    let mut wal = wal.lock().unwrap();
                    wal.append_log(log_entry);
                } // wal lock is dropped here
                
                println!("SET command executed for key: {}", key);
            }
            Ok(Command::GET { key }) => {
                let result = {
                    let mut db = db.lock().unwrap();
                    db.get(&key).map(|s| s.clone()) // Clone the String inside the Option
                }; // db lock is dropped here
                
                match result {
                    Some(value) => 
                    {
                        println!("GET command executed for key: {}, value: {}", key, value);
                        writer.write_all(format!("GET command executed for key: {}, value: {}", key, value).as_bytes()).await?;
                    },
                    None => println!("GET command executed for key: {}, but it does not exist or has expired", key),
                }
                
            }
            Ok(Command::DEL { key }) => {
                let deleted = {
                    let mut db = db.lock().unwrap();
                    db.del(&key)
                }; // db lock is dropped here
                
                {
                    let mut wal = wal.lock().unwrap();
                    wal.append_log(LogEntry::DEL { key: key.clone() });
                } // wal lock is dropped here
                
                if deleted {
                    println!("DEL command executed for key: {}", key);
                } else {
                    println!("DEL command executed for key: {}, but it does not exist", key);
                }
            }
            Err(e) => {
                eprintln!("Error parsing command: {}", e);
            }
        }
       
    }

    println!("Client disconnected.");
    Ok(())
}