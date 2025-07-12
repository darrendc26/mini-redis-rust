use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct DB {
    data: HashMap<String, Value>,
}

pub struct Value {
    data: String,
    expires_at: Option<Instant>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            data: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String, expires_in: Option<u64>) {
        let expires_at = expires_in.map(|secs| Instant::now() + Duration::from_secs(secs));
        self.data.insert(key, Value { data: value, expires_at });
    }
  
    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.data.get(key) {
            if let Some(expiry) = value.expires_at {
                if Instant::now() > expiry {
                    // Expired: remove and return None
                    self.data.remove(key);
                    return None;
                }
            }
            return Some(value.data.clone());
        }
        None
    }

    pub fn del(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}