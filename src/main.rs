use tokio;
use std:: { sync::{Arc, Mutex}, error::Error};
use wal::*;
use server::*;

pub mod server;
pub mod wal;
pub mod db;
pub mod parser;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Arc::new(Mutex::new(db::DB::new()));
    let wal_path = "wal.log";
    let wal = Arc::new(Mutex::new(WAL::new(wal_path)));
    run_server("127.0.0.1:3000", db.clone(), wal.clone()).await?;
    Ok(())
}

