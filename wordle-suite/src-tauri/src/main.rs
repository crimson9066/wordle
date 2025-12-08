// Main Tauri application entry point
// This is where the desktop app gets set up and started

mod engine;
mod word_list;
mod db;
mod hint_engine;
mod commands;

use tauri::{
    Config, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem, Window,
};
use std::sync::Mutex;

use db::Database;
use word_list::DailyWordConfig;
use commands::*;

// Shared state that persists across all IPC calls
// db stores the database connection
// daily_word_config handles the word list
pub struct AppState {
    pub db: Mutex<Option<Database>>,
    pub daily_word_config: DailyWordConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up the system tray menu (the icon in taskbar when minimized)
    let system_tray_menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::new("Show", "show".to_string()))
        .add_item(SystemTrayMenuItem::Separator)
        .add_item(SystemTrayMenuItem::new("Quit", "quit".to_string()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(handle_system_tray_event)
        // Manage the shared app state so all commands can access it
        .manage(AppState {
            db: Mutex::new(None),
            daily_word_config: DailyWordConfig::new(),
        })
        // Register all the IPC commands that the frontend can call
        .invoke_handler(tauri::generate_handler![
            eval_guess_cmd,
            get_daily_word_cmd,
            validate_guess_cmd,
            new_game_cmd,
            make_guess_cmd,
            get_hint_cmd,
            get_or_create_player_cmd,
            get_player_stats_cmd,
            init_db_cmd,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::WindowEvent { .. } => {}
            _ => {}
        });
}

fn handle_system_tray_event(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    // Handle clicking on the tray icon or tray menu items
    match event {
        tauri::SystemTrayEvent::LeftClick { .. } => {
            // Show the window when clicking the tray icon
            if let Ok(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                // Show menu item was clicked
                if let Ok(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                // User clicked Quit - exit the app
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

// IPC command wrappers - these get called from React via Tauri's invoke system

#[tauri::command]
fn eval_guess_cmd(request: commands::EvaluateGuessRequest) -> Result<serde_json::Value, String> {
    commands::cmd_evaluate_guess(request)
}

#[tauri::command]
fn get_daily_word_cmd(
    request: commands::GetDailyWordRequest,
    state: tauri::State<AppState>,
) -> Result<String, String> {
    commands::cmd_get_daily_word(&state.daily_word_config, request)
}

#[tauri::command]
fn validate_guess_cmd(
    word: String,
    state: tauri::State<AppState>,
) -> Result<bool, String> {
    commands::cmd_validate_guess(word, &state.daily_word_config)
}

#[tauri::command]
fn new_game_cmd(
    request: commands::NewGameRequest,
    state: tauri::State<AppState>,
) -> Result<db::GameState, String> {
    let db = state
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_new_game(request, db, &state.daily_word_config)
}

#[tauri::command]
fn make_guess_cmd(
    request: commands::GuessRequest,
    state: tauri::State<AppState>,
) -> Result<(serde_json::Value, Option<String>), String> {
    let db = state
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_make_guess(request, db)
}

#[tauri::command]
fn get_hint_cmd(
    request: commands::GetHintRequest,
    state: tauri::State<AppState>,
) -> Result<hint_engine::HintAnalysis, String> {
    let word_list = &state.daily_word_config.word_list;
    commands::cmd_get_hint(request, word_list)
}

#[tauri::command]
fn get_or_create_player_cmd(
    player_name: String,
    state: tauri::State<AppState>,
) -> Result<db::PlayerProfile, String> {
    let db = state
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_get_or_create_player(player_name, db)
}

#[tauri::command]
fn get_player_stats_cmd(
    player_id: String,
    state: tauri::State<AppState>,
) -> Result<db::GameStats, String> {
    let db = state
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_get_player_stats(player_id, db)
}

#[tauri::command]
fn init_db_cmd(
    app_handle: tauri::AppHandle,
    mut state: tauri::State<AppState>,
) -> Result<String, String> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;

    let db_path = data_dir.join("wordle.db");

    let db = Database::open(&db_path).map_err(|e| e.to_string())?;
    *state.db.lock().unwrap() = Some(db);

    Ok(format!("Database initialized at {:?}", db_path))
}
