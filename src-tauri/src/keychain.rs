use thiserror::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("Keychain access denied")]
    AccessDenied,
    #[error("Item not found")]
    ItemNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct KeychainManager {
    // For now, use in-memory storage
    // In production, this would use macOS Keychain
    storage: Arc<Mutex<HashMap<String, String>>>,
}

impl KeychainManager {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store_token(&self, account: &str, token: &str) -> Result<(), KeychainError> {
        let account_key = format!("github:{}", account);
        let mut storage = self.storage.lock().unwrap();
        storage.insert(account_key, token.to_string());
        Ok(())
    }

    pub fn get_token(&self, account: &str) -> Result<String, KeychainError> {
        let account_key = format!("github:{}", account);
        let storage = self.storage.lock().unwrap();
        storage.get(&account_key)
            .cloned()
            .ok_or(KeychainError::ItemNotFound)
    }

    pub fn delete_token(&self, account: &str) -> Result<(), KeychainError> {
        let account_key = format!("github:{}", account);
        let mut storage = self.storage.lock().unwrap();
        storage.remove(&account_key);
        Ok(())
    }

    pub fn list_tokens(&self) -> Result<Vec<String>, KeychainError> {
        let storage = self.storage.lock().unwrap();
        let accounts: Vec<String> = storage
            .keys()
            .filter(|key| key.starts_with("github:"))
            .map(|key| key.strip_prefix("github:").unwrap_or(key).to_string())
            .collect();
        Ok(accounts)
    }
}