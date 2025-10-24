use crate::database::{Account, Database};
use crate::github_auth::GitHubAuth;
use crate::keychain::KeychainManager;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub auth_method: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryMappingInfo {
    pub id: String,
    pub remote_url: String,
    pub account_id: String,
    pub remember: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeInfo {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub message: String,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHelperStatus {
    pub installed: bool,
    pub configured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSHKeyInfo {
    pub public_key: String,
    pub private_key_path: String,
    pub key_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSHConfig {
    pub host: String,
    pub hostname: String,
    pub user: String,
    pub identity_file: String,
}

#[tauri::command]
pub async fn get_accounts(db: State<'_, Database>) -> Result<Vec<AccountInfo>, String> {
    let accounts = db.get_accounts().map_err(|e| e.to_string())?;

    let account_infos: Vec<AccountInfo> = accounts
        .into_iter()
        .map(|account| AccountInfo {
            id: account.id,
            username: account.username,
            avatar_url: account.avatar_url,
            auth_method: account.auth_method,
            created_at: account.created_at.to_rfc3339(),
        })
        .collect();

    Ok(account_infos)
}

#[tauri::command]
pub async fn add_account(
    db: State<'_, Database>,
    keychain: State<'_, KeychainManager>,
    username: String,
    token: String,
) -> Result<AccountInfo, String> {
    // Validate token with GitHub API
    let github_auth = GitHubAuth::new();
    let user = github_auth
        .validate_token(&token)
        .await
        .map_err(|e| format!("Token validation failed: {}", e))?;

    // Check if account already exists
    if let Ok(Some(_)) = db.get_account_by_username(&username) {
        return Err("Account already exists".to_string());
    }

    // Store token in keychain
    keychain
        .store_token(&username, &token)
        .map_err(|e| format!("Failed to store token: {}", e))?;

    // Create account record
    let account = Account {
        id: Uuid::new_v4().to_string(),
        username: user.login,
        avatar_url: Some(user.avatar_url),
        auth_method: "manual".to_string(),
        created_at: Utc::now(),
    };

    db.add_account(&account).map_err(|e| e.to_string())?;

    Ok(AccountInfo {
        id: account.id,
        username: account.username,
        avatar_url: account.avatar_url,
        auth_method: account.auth_method,
        created_at: account.created_at.to_rfc3339(),
    })
}

#[tauri::command]
pub async fn remove_account(
    db: State<'_, Database>,
    keychain: State<'_, KeychainManager>,
    account_id: String,
) -> Result<(), String> {
    // Get account info first
    let accounts = db.get_accounts().map_err(|e| e.to_string())?;
    let account = accounts
        .iter()
        .find(|a| a.id == account_id)
        .ok_or("Account not found")?;

    // Remove from keychain
    keychain
        .delete_token(&account.username)
        .map_err(|e| format!("Failed to delete token: {}", e))?;

    // Remove from database
    db.remove_account(&account_id).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    keychain: State<'_, KeychainManager>,
    username: String,
) -> Result<TestConnectionResult, String> {
    let token = keychain
        .get_token(&username)
        .map_err(|e| format!("Failed to get token: {}", e))?;

    let github_auth = GitHubAuth::new();

    match github_auth.validate_token(&token).await {
        Ok(user) => {
            let scopes = github_auth.test_token_scopes(&token).await.ok();
            Ok(TestConnectionResult {
                success: true,
                message: format!("Connected as {}", user.login),
                scopes,
            })
        }
        Err(e) => Ok(TestConnectionResult {
            success: false,
            message: format!("Connection failed: {}", e),
            scopes: None,
        }),
    }
}

#[tauri::command]
pub async fn get_repository_mappings(
    db: State<'_, Database>,
) -> Result<Vec<RepositoryMappingInfo>, String> {
    let mappings = db.get_repository_mappings().map_err(|e| e.to_string())?;

    let mapping_infos: Vec<RepositoryMappingInfo> = mappings
        .into_iter()
        .map(|mapping| RepositoryMappingInfo {
            id: mapping.id,
            remote_url: mapping.remote_url,
            account_id: mapping.account_id,
            remember: mapping.remember,
            created_at: mapping.created_at.to_rfc3339(),
        })
        .collect();

    Ok(mapping_infos)
}

#[tauri::command]
pub async fn set_repository_mapping(
    db: State<'_, Database>,
    remote_url: String,
    account_id: String,
    remember: bool,
) -> Result<(), String> {
    db.set_repository_mapping(&remote_url, &account_id, remember)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_repository_mapping(
    db: State<'_, Database>,
    mapping_id: String,
) -> Result<(), String> {
    db.remove_repository_mapping(&mapping_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn install_git_helper() -> Result<(), String> {
    use std::process::Command;

    let current_exe =
        std::env::current_exe().map_err(|e| format!("Failed to get current executable: {}", e))?;
    let helper_command = format!("!{} credential-helper", current_exe.display());

    // Clear existing credential helpers
    let _ = Command::new("git")
        .args(["config", "--global", "--unset-all", "credential.helper"])
        .output();

    // Set our credential helper
    let output = Command::new("git")
        .args(["config", "--global", "credential.helper", &helper_command])
        .output()
        .map_err(|e| format!("Failed to run git config: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Git config failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_git_helper_status() -> Result<GitHelperStatus, String> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["config", "--global", "credential.helper"])
        .output()
        .map_err(|e| format!("Failed to run git config: {}", e))?;

    if !output.status.success() {
        return Ok(GitHelperStatus {
            installed: false,
            configured: false,
        });
    }

    let config = String::from_utf8_lossy(&output.stdout);
    let current_exe =
        std::env::current_exe().map_err(|e| format!("Failed to get current executable: {}", e))?;
    let expected_helper = format!("!{} credential-helper", current_exe.display());

    Ok(GitHelperStatus {
        installed: true,
        configured: config.trim() == expected_helper,
    })
}

#[tauri::command]
pub async fn generate_ssh_key(username: String) -> Result<SSHKeyInfo, String> {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    let home_dir = std::env::var("HOME").map_err(|_| "HOME directory not found")?;

    let ssh_dir = PathBuf::from(home_dir).join(".ssh");
    fs::create_dir_all(&ssh_dir).map_err(|e| format!("Failed to create SSH directory: {}", e))?;

    let key_name = format!("gitswitchhub_{}", username);
    let private_key_path = ssh_dir.join(&key_name);
    let public_key_path = ssh_dir.join(format!("{}.pub", key_name));

    // Generate SSH key
    let output = Command::new("ssh-keygen")
        .args([
            "-t",
            "ed25519",
            "-f",
            &private_key_path.to_string_lossy(),
            "-C",
            &format!("{}@gitswitchhub", username),
            "-N",
            "", // No passphrase
        ])
        .output()
        .map_err(|e| format!("Failed to generate SSH key: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "SSH key generation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Read public key
    let public_key = fs::read_to_string(&public_key_path)
        .map_err(|e| format!("Failed to read public key: {}", e))?;

    Ok(SSHKeyInfo {
        public_key: public_key.trim().to_string(),
        private_key_path: private_key_path.to_string_lossy().to_string(),
        key_id: key_name,
    })
}

#[tauri::command]
pub async fn get_ssh_config(username: String) -> Result<SSHConfig, String> {
    let home_dir = std::env::var("HOME").map_err(|_| "HOME directory not found")?;

    let key_name = format!("gitswitchhub_{}", username);
    let private_key_path = format!("{}/.ssh/{}", home_dir, key_name);

    Ok(SSHConfig {
        host: format!("github-{}", username),
        hostname: "github.com".to_string(),
        user: "git".to_string(),
        identity_file: private_key_path,
    })
}

#[tauri::command]
pub async fn convert_remote_to_ssh(remote_url: String, username: String) -> Result<String, String> {
    // Convert HTTPS URL to SSH format
    if remote_url.starts_with("https://github.com/") {
        let repo_path = remote_url
            .strip_prefix("https://github.com/")
            .ok_or("Invalid GitHub URL")?;
        Ok(format!("git@github-{}:{}", username, repo_path))
    } else {
        Err("Not a GitHub HTTPS URL".to_string())
    }
}

#[tauri::command]
pub async fn show_account_chooser(
    db: State<'_, Database>,
    keychain: State<'_, KeychainManager>,
    _repo_url: String,
) -> Result<String, String> {
    // Get all accounts
    let accounts = db.get_accounts().map_err(|e| e.to_string())?;

    if accounts.is_empty() {
        return Err("No accounts configured".to_string());
    }

    // For now, return the first account
    // In a real implementation, this would show a GUI chooser
    let account = &accounts[0];

    // Verify token exists
    keychain
        .get_token(&account.username)
        .map_err(|e| format!("No token found for account: {}", e))?;

    Ok(account.username.clone())
}
