use std::process::Command;
use std::path::PathBuf;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SSHError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process error: {0}")]
    Process(String),
    #[error("SSH key not found")]
    KeyNotFound,
}

pub struct SSHManager;

impl Default for SSHManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SSHManager {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_key(&self, username: &str) -> Result<SSHKeyInfo, SSHError> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| SSHError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME directory not found"
            )))?;
        
        let ssh_dir = PathBuf::from(home_dir).join(".ssh");
        fs::create_dir_all(&ssh_dir)?;
        
        let key_name = format!("gitswitchhub_{}", username);
        let private_key_path = ssh_dir.join(&key_name);
        let public_key_path = ssh_dir.join(format!("{}.pub", key_name));
        
        // Generate SSH key
        let output = Command::new("ssh-keygen")
              .args([
                "-t", "ed25519",
                "-f", &private_key_path.to_string_lossy(),
                "-C", &format!("{}@gitswitchhub", username),
                "-N", "", // No passphrase
            ])
            .output()?;
        
        if !output.status.success() {
            return Err(SSHError::Process(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        // Read public key
        let public_key = fs::read_to_string(&public_key_path)?;
        
        Ok(SSHKeyInfo {
            public_key: public_key.trim().to_string(),
            private_key_path: private_key_path.to_string_lossy().to_string(),
            key_id: key_name,
        })
    }

    pub fn get_ssh_config(&self, username: &str) -> Result<SSHConfig, SSHError> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| SSHError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME directory not found"
            )))?;
        
        let key_name = format!("gitswitchhub_{}", username);
        let private_key_path = format!("{}/.ssh/{}", home_dir, key_name);
        
        Ok(SSHConfig {
            host: format!("github-{}", username),
            hostname: "github.com".to_string(),
            user: "git".to_string(),
            identity_file: private_key_path,
        })
    }

    pub fn add_to_ssh_config(&self, username: &str) -> Result<(), SSHError> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| SSHError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME directory not found"
            )))?;
        
        let ssh_config_path = PathBuf::from(home_dir).join(".ssh").join("config");
        let config = self.get_ssh_config(username)?;
        
        let ssh_config_entry = format!(
            "\nHost {}\n\
             HostName {}\n\
             User {}\n\
             IdentityFile {}\n\
             IdentitiesOnly yes\n",
            config.host, config.hostname, config.user, config.identity_file
        );
        
        // Append to SSH config
        use std::io::Write;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&ssh_config_path)?
            .write_all(ssh_config_entry.as_bytes())?;
        
        Ok(())
    }

    pub fn remove_from_ssh_config(&self, username: &str) -> Result<(), SSHError> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| SSHError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME directory not found"
            )))?;
        
        let ssh_config_path = PathBuf::from(home_dir).join(".ssh").join("config");
        
        if !ssh_config_path.exists() {
            return Ok(()); // Nothing to remove
        }
        
        let content = fs::read_to_string(&ssh_config_path)?;
        let host_pattern = format!("github-{}", username);
        
        // Remove the host block
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();
        let mut skip_until_next_host = false;
        
        for line in lines {
            if line.trim().starts_with("Host ") && line.contains(&host_pattern) {
                skip_until_next_host = true;
                continue;
            }
            
            if skip_until_next_host {
                if line.trim().starts_with("Host ") {
                    skip_until_next_host = false;
                    new_lines.push(line);
                }
                // Skip lines until next Host
                continue;
            }
            
            new_lines.push(line);
        }
        
        fs::write(&ssh_config_path, new_lines.join("\n"))?;
        Ok(())
    }

    pub fn test_ssh_connection(&self, username: &str) -> Result<bool, SSHError> {
        let config = self.get_ssh_config(username)?;
        
        let output = Command::new("ssh")
              .args([
                "-T",
                "-o", "StrictHostKeyChecking=no",
                "-o", "ConnectTimeout=10",
                &format!("{}@{}", config.user, config.hostname),
            ])
            .output()?;
        
        // GitHub returns exit code 1 for successful SSH connections
        // but with a message about successful authentication
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(stderr.contains("successfully authenticated") || 
           stderr.contains("Hi") || 
           output.status.code() == Some(1))
    }
}

#[derive(Debug, Clone)]
pub struct SSHKeyInfo {
    pub public_key: String,
    pub private_key_path: String,
    pub key_id: String,
}

#[derive(Debug, Clone)]
pub struct SSHConfig {
    pub host: String,
    pub hostname: String,
    pub user: String,
    pub identity_file: String,
}
