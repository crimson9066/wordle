import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface GameState {
  id: string;
  player_id: string;
  game_mode: string;
  word_id: string;
  guesses: string[];
  feedback: number[][];
  result: "won" | "lost" | "in_progress";
  started_at: string;
  finished_at?: string;
}

interface PlayerProfile {
  id: string;
  display_name: string;
  created_at: string;
  settings: Record<string, unknown>;
  installs_seed: string;
}

interface PlayerStats {
  player_id: string;
  total_games: number;
  total_wins: number;
  current_streak: number;
  best_streak: number;
  avg_guesses: number;
}

function App() {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [currentGuess, setCurrentGuess] = useState("");
  const [feedback, setFeedback] = useState<number[][]>([]);
  const [playerStats, setPlayerStats] = useState<PlayerStats | null>(null);
  const [message, setMessage] = useState("");
  const [theme, setTheme] = useState("light");

  useEffect(() => {
    initializeApp();
  }, []);

  const initializeApp = async () => {
    try {
      // Initialize database
      await invoke("init_db_cmd");

      // Create or get player (for demo, use a default player)
      const player = (await invoke("get_or_create_player_cmd", {
        playerName: "Default Player",
      })) as PlayerProfile;

      // Create new game
      const game = (await invoke("new_game_cmd", {
        request: {
          player_id: player.id,
          game_mode: "Classic",
        },
      })) as GameState;

      setGameState(game);

      // Get player stats
      const stats = (await invoke("get_player_stats_cmd", {
        playerId: player.id,
      })) as PlayerStats;
      setPlayerStats(stats);
    } catch (error) {
      console.error("Failed to initialize app:", error);
      setMessage("Failed to initialize app");
    }
  };

  const handleKeyPress = async (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && currentGuess.length === 5) {
      await submitGuess();
    }
  };

  const submitGuess = async () => {
    if (!gameState || gameState.result !== "in_progress") {
      return;
    }

    try {
      const isValid = await invoke("validate_guess_cmd", {
        word: currentGuess,
      });

      if (!isValid) {
        setMessage("Not in word list");
        return;
      }

      const result = await invoke("make_guess_cmd", {
        request: {
          game_id: gameState.id,
          guess: currentGuess,
        },
      });

      const [response, error] = result as [any, string | null];

      setFeedback([...feedback, response.feedback]);
      setCurrentGuess("");

      if (error) {
        setMessage(error);
      } else if (response.is_correct) {
        setMessage("You won!");
      } else if (response.result === "lost") {
        setMessage(`Game Over! Remaining guesses: ${response.remaining_guesses}`);
      }

      const updated = (await invoke("get_game_state", {
        gameId: gameState.id,
      })) as GameState;
      setGameState(updated);
    } catch (error) {
      console.error("Error submitting guess:", error);
      setMessage("Error submitting guess");
    }
  };

  const toggleTheme = () => {
    setTheme(theme === "light" ? "dark" : "light");
  };

  return (
    <div className={`app ${theme}-theme`}>
      <header className="app-header">
        <h1>Wordle Suite</h1>
        <button onClick={toggleTheme} className="theme-toggle">
          {theme === "light" ? "Dark" : "Light"}
        </button>
      </header>

      <main className="app-main">
        <div className="game-container">
          <div className="game-board">
            {feedback.length > 0 && (
              <div className="previous-guesses">
                {feedback.map((row, idx) => (
                  <div key={idx} className="guess-row">
                    {row.map((letter, pos) => (
                      <div
                        key={pos}
                        className={`tile feedback-${letter}`}
                      >
                        {gameState && gameState.guesses[idx][pos]}
                      </div>
                    ))}
                  </div>
                ))}
              </div>
            )}

            <div className="current-guess">
              <input
                type="text"
                maxLength={5}
                value={currentGuess}
                onChange={(e) =>
                  setCurrentGuess(e.target.value.toUpperCase())
                }
                onKeyPress={handleKeyPress}
                placeholder="Enter 5-letter word"
                disabled={gameState?.result !== "in_progress"}
              />
              <button
                onClick={submitGuess}
                disabled={
                  currentGuess.length !== 5 ||
                  gameState?.result !== "in_progress"
                }
              >
                Submit
              </button>
            </div>

            {message && <div className="message">{message}</div>}
          </div>

          <aside className="stats-sidebar">
            <h2>Stats</h2>
            {playerStats && (
              <div className="stats">
                <div className="stat">
                  <span className="label">Total Games:</span>
                  <span className="value">{playerStats.total_games}</span>
                </div>
                <div className="stat">
                  <span className="label">Wins:</span>
                  <span className="value">{playerStats.total_wins}</span>
                </div>
                <div className="stat">
                  <span className="label">Streak:</span>
                  <span className="value">{playerStats.current_streak}</span>
                </div>
                <div className="stat">
                  <span className="label">Best Streak:</span>
                  <span className="value">{playerStats.best_streak}</span>
                </div>
                <div className="stat">
                  <span className="label">Avg Guesses:</span>
                  <span className="value">
                    {playerStats.avg_guesses.toFixed(2)}
                  </span>
                </div>
              </div>
            )}
          </aside>
        </div>
      </main>

      <footer className="app-footer">
        <p>Wordle Suite v0.1.0 | Built with Tauri, React and Rust</p>
      </footer>
    </div>
  );
}

export default App;
