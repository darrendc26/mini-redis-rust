# Mini Redis (Rust TCP Key-Value Store)

A lightweight Redis-inspired in-memory key-value store built in **Rust**.  
Supports basic `SET`, `GET`, `DEL` commands over a raw **TCP protocol**, with optional **key expiration** and **write-ahead logging (WAL)** for persistence.

---

## Features

-  In-memory key-value store (`HashMap`)
-  Redis-like command syntax:
  - `SET key value [EX seconds]`
  - `GET key`
  - `DEL key`
- Key expiration (`EX <seconds>`)
- Persistent Write-Ahead Log (WAL)
- WAL replay on restart (crash recovery)
- Async TCP server using `tokio`
- Minimal codebase with modular design

---

## Architecture Overview

Client (telnet/nc/custom)
        ↓
Async TCP Server (Tokio)
        ↓
Command Parser → Command Enum
        ↓
In-Memory DB <----> WAL
        ↓
Response Sent Back

---

## Commands

SET name Darren
GET name
DEL name

SET temp 42 EX 5 # key expires in 5 seconds
GET temp # after 5 seconds → (nil)

---


## Build & Run

### Requirements
- Rust & Cargo (https://rustup.rs)
- telnet or netcat (optional for testing)

### Run the server

```bash
cargo run
```

Server runs at 127.0.0.1:3000 by default.

### Testing 

## Test with `telnet`

```bash
telnet 127.0.0.1 3000
```

## Test with `netcat`

```bash
nc -v 127.0.0.1 3000
```

---

## Write-Ahead Logging (WAL)

The WAL is a log of all commands executed on the server. It is used to recover the server state in case of a crash or restart.

The WAL is persisted to a file named `wal.log` in the current directory.

Wal Format:

```json

{
    "SET":{
        "key":"name",
        "value":"darren",
        "expires_in":5
        }
}

```

---

## Author

**Darren Da Costa**