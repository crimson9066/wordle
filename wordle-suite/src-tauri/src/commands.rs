// Tauri IPC commands - bridge between React frontend and Rust backend
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{Database, DbError, GameState, GameStats, PlayerProfile};
use crate::engine::{evaluate_guess, LetterFeedback};
use crate::hint_engine::{HintAnalysis, HintEngine};
use crate::word_list::DailyWordConfig;

const MAX_GUESSES: usize = 6;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateGuessRequest {
    pub guess: String,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDailyWordRequest {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameRequest {
    pub player_id: String,
    pub game_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessRequest {
    pub game_id: String,
    pub player_id: String,
    pub guess: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHintRequest {
    pub game_id: String,
    pub player_id: String,
    pub remaining_words: Vec<String>,
}

/// Evaluate a guess against an answer
pub fn cmd_evaluate_guess(request: EvaluateGuessRequest) -> Result<serde_json::Value, String> {
    validate_word_shape(&request.guess)?;
    validate_word_shape(&request.answer)?;
    let evaluation = evaluate_guess(&request.guess, &request.answer);
    Ok(serde_json::to_value(evaluation).map_err(|e| e.to_string())?)
}

/// Get daily word for a specific date
pub fn cmd_get_daily_word(
    config: &DailyWordConfig,
    request: GetDailyWordRequest,
) -> Result<String, String> {
    config
        .get_daily_word(request.year, request.month, request.day)
        .ok_or_else(|| "No word list available".to_string())
}

/// Validate if a guess is a valid word
pub fn cmd_validate_guess(word: String, config: &DailyWordConfig) -> Result<bool, String> {
    Ok(is_valid_word(&word, config))
}

/// Create a new game
pub fn cmd_new_game(
    request: NewGameRequest,
    db: &Database,
    daily_word_config: &DailyWordConfig,
) -> Result<GameState, String> {
    if db
        .get_player(&request.player_id)
        .map_err(db_error)?
        .is_none()
    {
        return Err("Player not found".to_string());
    }
    if request.game_mode != "Classic" {
        return Err("Unsupported game mode".to_string());
    }

    let today = Utc::now();
    let answer = daily_word_config
        .get_daily_word(today.year() as u32, today.month(), today.day())
        .ok_or_else(|| "Failed to get daily word".to_string())?;

    let game = GameState {
        id: Uuid::new_v4().to_string(),
        player_id: request.player_id,
        game_mode: request.game_mode,
        word_id: answer.clone(),
        guesses: vec![],
        feedback: vec![],
        result: "in_progress".to_string(),
        started_at: Utc::now().to_rfc3339(),
        finished_at: None,
    };

    db.save_game_state(&game).map_err(db_error)?;

    Ok(game)
}

/// Make a guess in a game
pub fn cmd_make_guess(
    request: GuessRequest,
    db: &Database,
    daily_word_config: &DailyWordConfig,
) -> Result<(serde_json::Value, Option<String>), String> {
    let mut game = db
        .get_game_state(&request.game_id)
        .map_err(db_error)?
        .ok_or_else(|| "Game not found".to_string())?;

    if game.player_id != request.player_id {
        return Err("Game access denied".to_string());
    }
    if game.result != "in_progress" {
        return Err("Game already finished".to_string());
    }
    if game.guesses.len() >= MAX_GUESSES {
        return Err("Game already finished".to_string());
    }
    if !is_valid_word(&request.guess, daily_word_config) {
        return Err("Invalid guess".to_string());
    }

    let evaluation = evaluate_guess(&request.guess, &game.word_id);

    // Convert LetterFeedback to i32 for storage
    let feedback_vec: Vec<i32> = evaluation
        .feedback
        .iter()
        .map(|f| match f {
            LetterFeedback::Absent => 0,
            LetterFeedback::Present => 1,
            LetterFeedback::Correct => 2,
        })
        .collect();

    game.guesses.push(evaluation.guess.clone());
    game.feedback.push(feedback_vec);

    // Check win condition
    if evaluation.is_correct {
        game.result = "won".to_string();
        game.finished_at = Some(Utc::now().to_rfc3339());

        // Update stats
        let num_guesses = game.guesses.len();
        db.update_stats_after_game(&game.player_id, true, num_guesses)
            .map_err(db_error)?;
    } else if game.guesses.len() >= MAX_GUESSES {
        game.result = "lost".to_string();
        game.finished_at = Some(Utc::now().to_rfc3339());

        // Update stats
        db.update_stats_after_game(&game.player_id, false, MAX_GUESSES)
            .map_err(db_error)?;
    }

    db.save_game_state(&game).map_err(db_error)?;

    let response = serde_json::json!({
        "game_id": game.id,
        "guess": evaluation.guess,
        "feedback": evaluation.feedback,
        "is_correct": evaluation.is_correct,
        "result": game.result,
        "remaining_guesses": MAX_GUESSES - game.guesses.len()
    });

    Ok((response, None))
}

/// Get hint suggestions for current game state
pub fn cmd_get_hint(
    request: GetHintRequest,
    word_list: &[String],
    db: &Database,
) -> Result<HintAnalysis, String> {
    let game = db
        .get_game_state(&request.game_id)
        .map_err(db_error)?
        .ok_or_else(|| "Game not found".to_string())?;
    if game.player_id != request.player_id
        || game.result != "in_progress"
        || request.remaining_words.len() > word_list.len()
    {
        return Err("Invalid hint request".to_string());
    }
    let engine = HintEngine::new(word_list.to_vec());
    let analysis = engine.analyze(&request.remaining_words, &[]);
    Ok(analysis)
}

fn validate_word_shape(word: &str) -> Result<(), String> {
    if word.chars().count() != 5
        || !word
            .chars()
            .all(|character| character.is_ascii_alphabetic())
    {
        return Err("Words must contain exactly five ASCII letters".to_string());
    }
    Ok(())
}

fn is_valid_word(word: &str, config: &DailyWordConfig) -> bool {
    validate_word_shape(word).is_ok() && config.is_valid_guess(word)
}

fn db_error(_: DbError) -> String {
    "Internal database error".to_string()
}

/// Get or create player profile
pub fn cmd_get_or_create_player(
    player_name: String,
    db: &Database,
) -> Result<PlayerProfile, String> {
    if player_name.trim().is_empty() || player_name.chars().count() > 64 {
        return Err("Player name must be between 1 and 64 characters".to_string());
    }
    let player_id = Uuid::new_v4().to_string();

    let profile = PlayerProfile {
        id: player_id,
        display_name: player_name,
        created_at: Utc::now().to_rfc3339(),
        settings: serde_json::json!({
            "theme": "light",
            "language": "en",
            "hard_mode": false,
            "color_blind_mode": false
        }),
        installs_seed: uuid::Uuid::new_v4().to_string(),
    };

    db.create_player(&profile).map_err(db_error)?;

    Ok(profile)
}

/// Get player stats
pub fn cmd_get_player_stats(player_id: String, db: &Database) -> Result<GameStats, String> {
    if db.get_player(&player_id).map_err(db_error)?.is_none() {
        return Err("Player not found".to_string());
    }
    db.get_or_create_stats(&player_id).map_err(db_error)
}
