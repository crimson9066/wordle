// API client for communicating with Tauri backend

import { invoke } from "@tauri-apps/api/core";
import type {
  GuessEvaluation,
  GameState,
  GameStats,
  PlayerProfile,
  HintAnalysis,
} from "@/types";

export const gameApi = {
  /**
   * Evaluate a guess against an answer
   */
  async evaluateGuess(guess: string, answer: string): Promise<GuessEvaluation> {
    return invoke("eval_guess_cmd", {
      request: { guess, answer },
    });
  },

  /**
   * Get the daily word for a specific date
   */
  async getDailyWord(
    year: number,
    month: number,
    day: number
  ): Promise<string> {
    return invoke("get_daily_word_cmd", {
      request: { year, month, day },
    });
  },

  /**
   * Validate if a word is in the word list
   */
  async validateGuess(word: string): Promise<boolean> {
    return invoke("validate_guess_cmd", { word });
  },

  /**
   * Start a new game
   */
  async newGame(playerId: string, gameMode: string): Promise<GameState> {
    return invoke("new_game_cmd", {
      request: { player_id: playerId, game_mode: gameMode },
    });
  },

  /**
   * Submit a guess in an ongoing game
   */
  async makeGuess(
    gameId: string,
    guess: string
  ): Promise<[any, string | null]> {
    return invoke("make_guess_cmd", {
      request: { game_id: gameId, guess },
    });
  },

  /**
   * Get hint suggestions for the current game state
   */
  async getHint(
    gameId: string,
    remainingWords: string[]
  ): Promise<HintAnalysis> {
    return invoke("get_hint_cmd", {
      request: { game_id: gameId, remaining_words: remainingWords },
    });
  },

  /**
   * Get or create a player profile
   */
  async getOrCreatePlayer(playerName: string): Promise<PlayerProfile> {
    return invoke("get_or_create_player_cmd", { playerName });
  },

  /**
   * Get player statistics
   */
  async getPlayerStats(playerId: string): Promise<GameStats> {
    return invoke("get_player_stats_cmd", { playerId });
  },

  /**
   * Initialize the database
   */
  async initDatabase(): Promise<string> {
    return invoke("init_db_cmd");
  },
};

// Error handler utility
export const handleApiError = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  return "An unknown error occurred";
};
