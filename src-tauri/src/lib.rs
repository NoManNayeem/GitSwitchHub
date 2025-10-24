pub mod database;
pub mod keychain;
pub mod git_helper;
pub mod github_auth;
pub mod ssh;
pub mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_accounts,
            commands::add_account,
            commands::remove_account,
            commands::test_connection,
            commands::get_repository_mappings,
            commands::set_repository_mapping,
            commands::remove_repository_mapping,
            commands::install_git_helper,
            commands::get_git_helper_status,
            commands::generate_ssh_key,
            commands::get_ssh_config,
            commands::convert_remote_to_ssh,
            commands::show_account_chooser
        ])
        .setup(|app| {
            // Initialize database on startup
            let db = database::Database::new()?;
            app.manage(db);
            
            // Initialize keychain manager
            let keychain = keychain::KeychainManager::new();
            app.manage(keychain);
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
