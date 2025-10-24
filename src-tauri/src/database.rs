use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub auth_method: String, // "device_flow" or "manual"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryMapping {
    pub id: String,
    pub remote_url: String,
    pub account_id: String,
    pub remember: bool,
    pub created_at: DateTime<Utc>,
}

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> Result<Self, DatabaseError> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;
        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf, DatabaseError> {
        let home_dir = std::env::var("HOME").map_err(|_| {
            DatabaseError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME directory not found",
            ))
        })?;

        let app_dir = PathBuf::from(home_dir).join(".gitswitchhub");
        std::fs::create_dir_all(&app_dir)?;

        Ok(app_dir.join("database.db"))
    }

    fn init_tables(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Create accounts table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                avatar_url TEXT,
                auth_method TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // Create repository_mappings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS repository_mappings (
                id TEXT PRIMARY KEY,
                remote_url TEXT NOT NULL,
                account_id TEXT NOT NULL,
                remember BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (account_id) REFERENCES accounts (id)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn add_account(&self, account: &Account) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO accounts (id, username, avatar_url, auth_method, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &account.id,
                &account.username,
                &account.avatar_url.as_deref().unwrap_or("").to_string(),
                &account.auth_method,
                &account.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, avatar_url, auth_method, created_at FROM accounts ORDER BY created_at DESC"
        )?;

        let account_iter = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                username: row.get(1)?,
                avatar_url: row.get::<_, Option<String>>(2)?.filter(|s| !s.is_empty()),
                auth_method: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut accounts = Vec::new();
        for account in account_iter {
            accounts.push(account?);
        }
        Ok(accounts)
    }

    pub fn get_account_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Account>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, avatar_url, auth_method, created_at FROM accounts WHERE username = ?1"
        )?;

        let mut rows = stmt.query_map([username], |row| {
            Ok(Account {
                id: row.get(0)?,
                username: row.get(1)?,
                avatar_url: row.get::<_, Option<String>>(2)?.filter(|s| !s.is_empty()),
                auth_method: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        if let Some(account) = rows.next() {
            Ok(Some(account?))
        } else {
            Ok(None)
        }
    }

    pub fn remove_account(&self, account_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Remove repository mappings first
        conn.execute(
            "DELETE FROM repository_mappings WHERE account_id = ?1",
            [account_id],
        )?;

        // Remove account
        conn.execute("DELETE FROM accounts WHERE id = ?1", [account_id])?;

        Ok(())
    }

    pub fn set_repository_mapping(
        &self,
        remote_url: &str,
        account_id: &str,
        remember: bool,
    ) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mapping_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        // Remove existing mapping for this URL
        conn.execute(
            "DELETE FROM repository_mappings WHERE remote_url = ?1",
            [remote_url],
        )?;

        // Add new mapping
        conn.execute(
            "INSERT INTO repository_mappings (id, remote_url, account_id, remember, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &mapping_id,
                remote_url,
                account_id,
                &remember.to_string(),
                &now.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get_repository_mapping(
        &self,
        remote_url: &str,
    ) -> Result<Option<RepositoryMapping>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, remote_url, account_id, remember, created_at FROM repository_mappings WHERE remote_url = ?1"
        )?;

        let mut rows = stmt.query_map([remote_url], |row| {
            Ok(RepositoryMapping {
                id: row.get(0)?,
                remote_url: row.get(1)?,
                account_id: row.get(2)?,
                remember: row.get::<_, i64>(3)? != 0,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        if let Some(mapping) = rows.next() {
            Ok(Some(mapping?))
        } else {
            Ok(None)
        }
    }

    pub fn get_repository_mappings(&self) -> Result<Vec<RepositoryMapping>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, remote_url, account_id, remember, created_at FROM repository_mappings ORDER BY created_at DESC"
        )?;

        let mapping_iter = stmt.query_map([], |row| {
            Ok(RepositoryMapping {
                id: row.get(0)?,
                remote_url: row.get(1)?,
                account_id: row.get(2)?,
                remember: row.get::<_, i64>(3)? != 0,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut mappings = Vec::new();
        for mapping in mapping_iter {
            mappings.push(mapping?);
        }
        Ok(mappings)
    }

    pub fn remove_repository_mapping(&self, mapping_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM repository_mappings WHERE id = ?1",
            [mapping_id],
        )?;
        Ok(())
    }
}
