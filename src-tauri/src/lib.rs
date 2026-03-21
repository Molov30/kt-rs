mod commands;
mod state;

use commands::{
    clear_logs, disable_obsolete, enable_debug, kill_ui, restart_service, send_notification,
    start_service, stop_service, watch_dumps,
};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            restart_service,
            start_service,
            stop_service,
            kill_ui,
            enable_debug,
            disable_obsolete,
            clear_logs,
            send_notification,
        ])
        .setup(|app| {
            watch_dumps(app.handle().clone());
            Ok(())
        })
        .manage(AppState::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
