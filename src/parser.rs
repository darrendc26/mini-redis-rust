#[derive(Debug, Clone)]
    pub enum Command {
    SET {
        key: String,
        value: String,
        expires_in: Option<u64>,
    },
    GET {
        key: String,
    },
    DEL {
        key: String,
    },
}  

pub fn parse(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.split(" ").collect();
    let command = parts[0].to_uppercase();
    match command.as_str() {
        "SET" => {
            if parts.len() < 3 {
                return Err("SET command requires at least key and value".to_string());
            }
            let key = parts[1];
            let value = parts[2];
            
            if parts.len() == 5 && parts[3].to_uppercase() == "EX" {
                let Ok(secs) = parts[4].parse::<u64>() else {
                    return Err("Invalid expiration time".to_string());
                };
               let expires_in = Some(secs);
                return Ok(Command::SET {
                    key: key.to_string(),
                    value: value.to_string(),
                    expires_in,
                });
            }else{
                return Ok(Command::SET {
                    key: key.to_string(),
                    value: value.to_string(),
                    expires_in: None, // Default to no expiration
                });
            }
            }
        "GET" => {
            if parts.len() != 2 {
                return Err("GET command requires exactly one key".to_string());
            }
            let key = parts[1];
            return Ok(Command::GET {
                key: key.to_string(),
            });
        }
        "DEL" => {
            if parts.len() != 2 {
                return Err("DEL command requires exactly one key".to_string());
            }
            let key = parts[1];
            return Ok(Command::DEL {
                key: key.to_string(),
            });
        }
        _ => Err(format!("Unknown command: {}", command)),
    }
}