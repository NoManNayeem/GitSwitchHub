use crate::database::Database;
use crate::keychain::KeychainManager;
use std::io::{self, BufRead};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitHelperError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Database error: {0}")]
    Database(#[from] crate::database::DatabaseError),
    #[error("Keychain error: {0}")]
    Keychain(#[from] crate::keychain::KeychainError),
    #[error("Process error: {0}")]
    Process(String),
}

pub struct GitCredentialHelper {
    db: Database,
    keychain: KeychainManager,
}

impl GitCredentialHelper {
    pub fn new(db: Database, keychain: KeychainManager) -> Self {
        Self { db, keychain }
    }

    pub fn run(&self) -> Result<(), GitHelperError> {
        let stdin = io::stdin();
        let lines = stdin.lock().lines();

        let mut url = String::new();
        let mut protocol = String::new();
        let mut host = String::new();
        let mut _path = String::new();

        // Parse Git credential helper input
        for line in lines {
            let line = line?;
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key {
                    "url" => url = value.to_string(),
                    "protocol" => protocol = value.to_string(),
                    "host" => host = value.to_string(),
                    "path" => _path = value.to_string(),
                    _ => {}
                }
            }
        }

        // Determine the repository URL
        let repo_url = if !url.is_empty() {
            url
        } else if !protocol.is_empty() && !host.is_empty() {
            format!("{}://{}", protocol, host)
        } else {
            return Err(GitHelperError::Process(
                "No repository URL found".to_string(),
            ));
        };

        // Check if we have a remembered account for this repository
        if let Some(mapping) = self.db.get_repository_mapping(&repo_url)? {
            if let Some(account) = self.db.get_account_by_username(&mapping.account_id)? {
                if let Ok(token) = self.keychain.get_token(&account.username) {
                    // Return credentials to Git
                    println!("username={}", account.username);
                    println!("password={}", token);
                    return Ok(());
                }
            }
        }

        // No remembered account, need to show account chooser
        self.show_account_chooser(&repo_url)?;

        Ok(())
    }

    fn show_account_chooser(&self, _repo_url: &str) -> Result<(), GitHelperError> {
        // Get all available accounts
        let accounts = self.db.get_accounts()?;

        if accounts.is_empty() {
            return Err(GitHelperError::Process(
                "No GitHub accounts configured".to_string(),
            ));
        }

        // For now, we'll use the first account as a fallback
        // In a real implementation, this would spawn a GUI window
        // For CLI mode, we'll need to implement a simple text-based chooser
        let account = &accounts[0];

        if let Ok(token) = self.keychain.get_token(&account.username) {
            println!("username={}", account.username);
            println!("password={}", token);
        } else {
            return Err(GitHelperError::Process(
                "No token found for account".to_string(),
            ));
        }

        Ok(())
    }

    pub fn install_git_helper(&self) -> Result<(), GitHelperError> {
        let current_exe = std::env::current_exe()?;
        let helper_command = format!("!{} credential-helper", current_exe.display());

        // Clear existing credential helpers
        let _ = Command::new("git")
            .args(["config", "--global", "--unset-all", "credential.helper"])
            .output();

        // Set our credential helper
        let output = Command::new("git")
            .args(["config", "--global", "credential.helper", &helper_command])
            .output()?;

        if !output.status.success() {
            return Err(GitHelperError::Process(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn get_git_helper_status(&self) -> Result<bool, GitHelperError> {
        let output = Command::new("git")
            .args(["config", "--global", "credential.helper"])
            .output()?;

        if !output.status.success() {
            return Ok(false);
        }

        let config = String::from_utf8_lossy(&output.stdout);
        let current_exe = std::env::current_exe()?;
        let expected_helper = format!("!{} credential-helper", current_exe.display());

        Ok(config.trim() == expected_helper)
    }
}
