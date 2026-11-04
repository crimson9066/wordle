// Main Tauri application entry point
// This is where the desktop app gets set up and started

mod commands;
mod db;
mod engine;
mod hint_engine;
mod word_list;

use std::sync::Mutex;
use tauri::Manager;

use db::Database;
use word_list::DailyWordConfig;

// Shared state that persists across all IPC calls
// db stores the database connection
// daily_word_config handles the word list
pub struct AppState {
    pub db: Mutex<Option<Database>>,
    pub daily_word_config: DailyWordConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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

fn main() {
    run();
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
fn validate_guess_cmd(word: String, state: tauri::State<AppState>) -> Result<bool, String> {
    commands::cmd_validate_guess(word, &state.daily_word_config)
}

#[tauri::command]
fn new_game_cmd(
    request: commands::NewGameRequest,
    state: tauri::State<AppState>,
) -> Result<db::GameState, String> {
    let db_guard = state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())?;
    let db = db_guard
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_new_game(request, db, &state.daily_word_config)
}

#[tauri::command]
fn make_guess_cmd(
    request: commands::GuessRequest,
    state: tauri::State<AppState>,
) -> Result<(serde_json::Value, Option<String>), String> {
    let db_guard = state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())?;
    let db = db_guard
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_make_guess(request, db, &state.daily_word_config)
}

#[tauri::command]
fn get_hint_cmd(
    request: commands::GetHintRequest,
    state: tauri::State<AppState>,
) -> Result<hint_engine::HintAnalysis, String> {
    let word_list = &state.daily_word_config.word_list;
    let db_guard = state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())?;
    let db = db_guard
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    commands::cmd_get_hint(request, word_list, db)
}

#[tauri::command]
fn get_or_create_player_cmd(
    player_name: String,
    state: tauri::State<AppState>,
) -> Result<db::PlayerProfile, String> {
    let db_guard = state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())?;
    let db = db_guard
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_get_or_create_player(player_name, db)
}

#[tauri::command]
fn get_player_stats_cmd(
    player_id: String,
    state: tauri::State<AppState>,
) -> Result<db::GameStats, String> {
    let db_guard = state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())?;
    let db = db_guard
        .as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    commands::cmd_get_player_stats(player_id, db)
}

#[tauri::command]
fn init_db_cmd(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
) -> Result<String, String> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;

    let db_path = data_dir.join("wordle.db");

    let db = Database::open(&db_path)
        .map_err(|_| "Failed to initialize database".to_string())?;
    *state
        .db
        .lock()
        .map_err(|_| "Database state unavailable".to_string())? = Some(db);

    Ok("Database initialized".to_string())
}
