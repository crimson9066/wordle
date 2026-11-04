// SQLite persistence layer
// Handles storing game states, player profiles, and player statistics

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    SqlError(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Invalid game state")]
    InvalidGameState,
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub id: String,
    pub display_name: String,
    pub created_at: String,
    pub settings: serde_json::Value,
    pub installs_seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub player_id: String,
    pub game_mode: String,
    pub word_id: String,
    pub guesses: Vec<String>,
    pub feedback: Vec<Vec<i32>>, // Feedback stored as numbers: 0=Absent, 1=Present, 2=Correct
    pub result: String,          // "won", "lost", or "in_progress"
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub player_id: String,
    pub total_games: i32,
    pub total_wins: i32,
    pub current_streak: i32,
    pub best_streak: i32,
    pub avg_guesses: f64,
    pub guess_distribution: [i32; 6], // Tracks wins by number of guesses (1-6)
    pub last_game_date: Option<String>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create a database at the given file path
    /// Creates the schema tables if they don't exist yet
    pub fn open<P: AsRef<Path>>(path: P) -> DbResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.run_migrations()?;
        Ok(db)
    }

    /// Create all the necessary tables if they don't exist
    /// Called when database is first opened
    fn run_migrations(&self) -> DbResult<()> {
        // Store player profiles - their name, settings, etc
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                settings TEXT NOT NULL,
                installs_seed TEXT NOT NULL
            )",
            [],
        )?;

        // Store individual game records - every game they played
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS game_states (
                id TEXT PRIMARY KEY,
                player_id TEXT NOT NULL,
                game_mode TEXT NOT NULL,
                word_id TEXT NOT NULL,
                guesses TEXT NOT NULL,
                feedback TEXT NOT NULL,
                result TEXT NOT NULL,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                FOREIGN KEY(player_id) REFERENCES players(id)
            )",
            [],
        )?;

        // Store aggregate stats - wins, streaks, averages
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS game_stats (
                player_id TEXT PRIMARY KEY,
                total_games INTEGER NOT NULL,
                total_wins INTEGER NOT NULL,
                current_streak INTEGER NOT NULL,
                best_streak INTEGER NOT NULL,
                avg_guesses REAL NOT NULL,
                guess_distribution TEXT NOT NULL,
                last_game_date TEXT,
                FOREIGN KEY(player_id) REFERENCES players(id)
            )",
            [],
        )?;

        // Event log for tracking what happened during games (optional analytics)
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS event_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                event_data TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY(player_id) REFERENCES players(id)
            )",
            [],
        )?;

        Ok(())
    }

    /// Create a new player
    pub fn create_player(&self, profile: &PlayerProfile) -> DbResult<()> {
        self.conn.execute(
            "INSERT INTO players (id, display_name, created_at, settings, installs_seed)
             VALUES (?, ?, ?, ?, ?)",
            params![
                &profile.id,
                &profile.display_name,
                &profile.created_at,
                serde_json::to_string(&profile.settings)?,
                &profile.installs_seed,
            ],
        )?;
        Ok(())
    }

    /// Get player profile
    pub fn get_player(&self, player_id: &str) -> DbResult<Option<PlayerProfile>> {
        let result = self.conn.query_row(
            "SELECT id, display_name, created_at, settings, installs_seed FROM players WHERE id = ?",
            params![player_id],
            |row| {
                let settings_str: String = row.get(3)?;
                Ok(PlayerProfile {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    created_at: row.get(2)?,
                    settings: serde_json::from_str(&settings_str)
                        .unwrap_or(serde_json::Value::Object(Default::default())),
                    installs_seed: row.get(4)?,
                })
            },
        ).optional()?;

        Ok(result)
    }

    /// Save game state
    pub fn save_game_state(&self, game: &GameState) -> DbResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO game_states
             (id, player_id, game_mode, word_id, guesses, feedback, result, started_at, finished_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &game.id,
                &game.player_id,
                &game.game_mode,
                &game.word_id,
                serde_json::to_string(&game.guesses)?,
                serde_json::to_string(&game.feedback)?,
                &game.result,
                &game.started_at,
                &game.finished_at,
            ],
        )?;
        Ok(())
    }

    /// Get game state
    pub fn get_game_state(&self, game_id: &str) -> DbResult<Option<GameState>> {
        let result = self.conn.query_row(
            "SELECT id, player_id, game_mode, word_id, guesses, feedback, result, started_at, finished_at
             FROM game_states WHERE id = ?",
            params![game_id],
            |row| {
                let guesses_str: String = row.get(4)?;
                let feedback_str: String = row.get(5)?;
                Ok(GameState {
                    id: row.get(0)?,
                    player_id: row.get(1)?,
                    game_mode: row.get(2)?,
                    word_id: row.get(3)?,
                    guesses: serde_json::from_str(&guesses_str)
                        .unwrap_or_default(),
                    feedback: serde_json::from_str(&feedback_str)
                        .unwrap_or_default(),
                    result: row.get(6)?,
                    started_at: row.get(7)?,
                    finished_at: row.get(8)?,
                })
            },
        ).optional()?;

        Ok(result)
    }

    /// Get or create player stats
    pub fn get_or_create_stats(&self, player_id: &str) -> DbResult<GameStats> {
        if let Some(stats) = self.get_stats(player_id)? {
            return Ok(stats);
        }

        let stats = GameStats {
            player_id: player_id.to_string(),
            total_games: 0,
            total_wins: 0,
            current_streak: 0,
            best_streak: 0,
            avg_guesses: 0.0,
            guess_distribution: [0; 6],
            last_game_date: None,
        };

        self.conn.execute(
            "INSERT INTO game_stats
             (player_id, total_games, total_wins, current_streak, best_streak, avg_guesses, guess_distribution)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                player_id,
                stats.total_games,
                stats.total_wins,
                stats.current_streak,
                stats.best_streak,
                stats.avg_guesses,
                serde_json::to_string(&stats.guess_distribution)?,
            ],
        )?;

        Ok(stats)
    }

    /// Get player stats
    pub fn get_stats(&self, player_id: &str) -> DbResult<Option<GameStats>> {
        let result = self.conn.query_row(
            "SELECT player_id, total_games, total_wins, current_streak, best_streak, avg_guesses, guess_distribution, last_game_date
             FROM game_stats WHERE player_id = ?",
            params![player_id],
            |row| {
                let dist_str: String = row.get(6)?;
                Ok(GameStats {
                    player_id: row.get(0)?,
                    total_games: row.get(1)?,
                    total_wins: row.get(2)?,
                    current_streak: row.get(3)?,
                    best_streak: row.get(4)?,
                    avg_guesses: row.get(5)?,
                    guess_distribution: serde_json::from_str(&dist_str)
                        .unwrap_or([0; 6]),
                    last_game_date: row.get(7)?,
                })
            },
        ).optional()?;

        Ok(result)
    }

    /// Update stats after a game
    pub fn update_stats_after_game(
        &self,
        player_id: &str,
        won: bool,
        guesses: usize,
    ) -> DbResult<()> {
        let mut stats = self.get_or_create_stats(player_id)?;

        stats.total_games += 1;
        if won {
            stats.total_wins += 1;
            stats.current_streak += 1;
            if stats.current_streak > stats.best_streak {
                stats.best_streak = stats.current_streak;
            }
            if guesses <= 6 {
                stats.guess_distribution[guesses - 1] += 1;
            }
        } else {
            stats.current_streak = 0;
        }

        let total_guesses: i32 = stats.guess_distribution.iter().sum();
        stats.avg_guesses = if stats.total_wins > 0 {
            total_guesses as f64 / stats.total_wins as f64
        } else {
            0.0
        };

        stats.last_game_date = Some(Utc::now().to_rfc3339());

        self.conn.execute(
            "UPDATE game_stats SET total_games = ?, total_wins = ?, current_streak = ?,
             best_streak = ?, avg_guesses = ?, guess_distribution = ?, last_game_date = ?
             WHERE player_id = ?",
            params![
                stats.total_games,
                stats.total_wins,
                stats.current_streak,
                stats.best_streak,
                stats.avg_guesses,
                serde_json::to_string(&stats.guess_distribution)?,
                &stats.last_game_date,
                player_id,
            ],
        )?;

        Ok(())
    }

    /// Log an event for analytics
    pub fn log_event(
        &self,
        player_id: &str,
        event_type: &str,
        event_data: &serde_json::Value,
    ) -> DbResult<()> {
        self.conn.execute(
            "INSERT INTO event_log (player_id, event_type, event_data, timestamp)
             VALUES (?, ?, ?, ?)",
            params![
                player_id,
                event_type,
                serde_json::to_string(event_data)?,
                Utc::now().to_rfc3339(),
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_creation() {
        let path = "test_wordle.db";
        let _ = fs::remove_file(path);

        let db = Database::open(path).unwrap();
        let profile = PlayerProfile {
            id: "test-player".to_string(),
            display_name: "Test Player".to_string(),
            created_at: Utc::now().to_rfc3339(),
            settings: serde_json::json!({}),
            installs_seed: "seed123".to_string(),
        };

        db.create_player(&profile).unwrap();
        let retrieved = db.get_player("test-player").unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().display_name, "Test Player");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_game_stats() {
        let path = "test_stats.db";
        let _ = fs::remove_file(path);

        let db = Database::open(path).unwrap();
        let profile = PlayerProfile {
            id: "player1".to_string(),
            display_name: "Test Player".to_string(),
            created_at: Utc::now().to_rfc3339(),
            settings: serde_json::json!({}),
            installs_seed: "seed123".to_string(),
        };
        db.create_player(&profile).unwrap();
        let stats = db.get_or_create_stats("player1").unwrap();
        assert_eq!(stats.total_games, 0);

        db.update_stats_after_game("player1", true, 4).unwrap();
        let stats = db.get_stats("player1").unwrap().unwrap();
        assert_eq!(stats.total_games, 1);
        assert_eq!(stats.total_wins, 1);

        let _ = fs::remove_file(path);
    }
}
