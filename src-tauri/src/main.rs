// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gitswitchhub_lib::database::Database;
use gitswitchhub_lib::git_helper::GitCredentialHelper;
use gitswitchhub_lib::keychain::KeychainManager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we're being called as a Git credential helper
    if args.len() > 1 && args[1] == "credential-helper" {
        // Run in CLI mode for Git credential helper
        let db = Database::new().expect("Failed to initialize database");
        let keychain = KeychainManager::new();
        let helper = GitCredentialHelper::new(db, keychain);

        if let Err(e) = helper.run() {
            eprintln!("GitSwitchHub credential helper error: {}", e);
            std::process::exit(1);
        }
    } else {
        // Run in GUI mode
        gitswitchhub_lib::run()
    }
}
