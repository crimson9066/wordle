// Tauri IPC commands - bridge between React frontend and Rust backend
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::engine::{evaluate_guess, LetterFeedback};
use crate::word_list::DailyWordConfig;
use crate::db::{Database, PlayerProfile, GameState, GameStats};
use crate::hint_engine::{HintEngine, HintAnalysis};

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
    pub guess: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHintRequest {
    pub game_id: String,
    pub remaining_words: Vec<String>,
}

/// Evaluate a guess against an answer
pub fn cmd_evaluate_guess(
    request: EvaluateGuessRequest,
) -> Result<serde_json::Value, String> {
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
    Ok(config.is_valid_guess(&word))
}

/// Create a new game
pub fn cmd_new_game(
    request: NewGameRequest,
    db: &Database,
    daily_word_config: &DailyWordConfig,
) -> Result<GameState, String> {
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

    db.save_game_state(&game)
        .map_err(|e| e.to_string())?;

    Ok(game)
}

/// Make a guess in a game
pub fn cmd_make_guess(
    request: GuessRequest,
    db: &Database,
) -> Result<(serde_json::Value, Option<String>), String> {
    let mut game = db
        .get_game_state(&request.game_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Game not found".to_string())?;

    if game.result != "in_progress" {
        return Err("Game already finished".to_string());
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
        let _ = db.update_stats_after_game(&game.player_id, true, num_guesses);
    } else if game.guesses.len() >= 6 {
        game.result = "lost".to_string();
        game.finished_at = Some(Utc::now().to_rfc3339());
        
        // Update stats
        let _ = db.update_stats_after_game(&game.player_id, false, 6);
    }

    db.save_game_state(&game)
        .map_err(|e| e.to_string())?;

    let response = serde_json::json!({
        "game_id": game.id,
        "guess": evaluation.guess,
        "feedback": evaluation.feedback,
        "is_correct": evaluation.is_correct,
        "result": game.result,
        "remaining_guesses": 6 - game.guesses.len()
    });

    let error_msg = if game.result == "lost" {
        Some(format!("The word was: {}", game.word_id))
    } else {
        None
    };

    Ok((response, error_msg))
}

/// Get hint suggestions for current game state
pub fn cmd_get_hint(
    request: GetHintRequest,
    word_list: &[String],
) -> Result<HintAnalysis, String> {
    let engine = HintEngine::new(word_list.to_vec());
    let analysis = engine.analyze(&request.remaining_words, &[]);
    Ok(analysis)
}

/// Get or create player profile
pub fn cmd_get_or_create_player(
    player_name: String,
    db: &Database,
) -> Result<PlayerProfile, String> {
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

    db.create_player(&profile)
        .map_err(|e| e.to_string())?;

    Ok(profile)
}

/// Get player stats
pub fn cmd_get_player_stats(
    player_id: String,
    db: &Database,
) -> Result<GameStats, String> {
    db.get_or_create_stats(&player_id)
        .map_err(|e| e.to_string())
}
