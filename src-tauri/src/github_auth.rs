use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum GitHubAuthError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Device flow timeout")]
    Timeout,
    #[error("Device flow denied")]
    Denied,
    #[error("Invalid token")]
    InvalidToken,
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct GitHubAuth {
    client: Client,
}

impl Default for GitHubAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHubAuth {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn start_device_flow(&self) -> Result<DeviceCodeResponse, GitHubAuthError> {
        let client_id = "Ov23liA2BpF0gI3E4nUX"; // GitHub OAuth App ID for GitSwitchHub

        let response = self
            .client
            .post("https://github.com/login/device/code")
            .header("Accept", "application/json")
            .form(&[("client_id", client_id), ("scope", "repo,user")])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubAuthError::Http(
                response.error_for_status().unwrap_err(),
            ));
        }

        let device_response: DeviceCodeResponse = response.json().await?;
        Ok(device_response)
    }

    pub async fn poll_for_token(
        &self,
        device_code: &str,
    ) -> Result<DeviceTokenResponse, GitHubAuthError> {
        let client_id = "Ov23liA2BpF0gI3E4nUX";
        let max_attempts = 60; // 5 minutes with 5-second intervals
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                return Err(GitHubAuthError::Timeout);
            }

            let response = self
                .client
                .post("https://github.com/login/oauth/access_token")
                .header("Accept", "application/json")
                .form(&[
                    ("client_id", client_id),
                    ("device_code", device_code),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ])
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(GitHubAuthError::Http(
                    response.error_for_status().unwrap_err(),
                ));
            }

            let text = response.text().await?;

            // Check for error responses
            if text.contains("authorization_pending") {
                attempts += 1;
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            if text.contains("access_denied") {
                return Err(GitHubAuthError::Denied);
            }

            if text.contains("expired_token") {
                return Err(GitHubAuthError::Timeout);
            }

            // Try to parse as successful response
            if let Ok(token_response) = serde_json::from_str::<DeviceTokenResponse>(&text) {
                return Ok(token_response);
            }

            attempts += 1;
            sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<GitHubUser, GitHubAuthError> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .header("Authorization", &format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "GitSwitchHub/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubAuthError::InvalidToken);
        }

        let user: GitHubUser = response.json().await?;
        Ok(user)
    }

    pub async fn test_token_scopes(&self, token: &str) -> Result<Vec<String>, GitHubAuthError> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .header("Authorization", &format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "GitSwitchHub/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitHubAuthError::InvalidToken);
        }

        // Extract scopes from response headers
        let scopes = response
            .headers()
            .get("X-OAuth-Scopes")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        Ok(scopes)
    }

    pub async fn check_sso_requirement(
        &self,
        token: &str,
        org: &str,
    ) -> Result<bool, GitHubAuthError> {
        let response = self
            .client
            .get(format!(
                "https://api.github.com/orgs/{}/memberships/me",
                org
            ))
            .header("Authorization", &format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "GitSwitchHub/1.0")
            .send()
            .await?;

        // If we get a 401 or 403, it might be due to SSO requirement
        Ok(!response.status().is_success())
    }
}
