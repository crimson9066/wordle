// Type definitions for the Wordle Suite

export enum LetterFeedback {
  Absent = 0,
  Present = 1,
  Correct = 2,
}

export interface GuessEvaluation {
  guess: string;
  feedback: LetterFeedback[];
  is_correct: boolean;
}

export interface PlayerProfile {
  id: string;
  display_name: string;
  created_at: string;
  settings: PlayerSettings;
  installs_seed: string;
}

export interface PlayerSettings {
  theme: "light" | "dark";
  language: string;
  hard_mode: boolean;
  color_blind_mode: boolean;
  [key: string]: any;
}

export interface GameState {
  id: string;
  player_id: string;
  game_mode: string;
  word_id: string;
  guesses: string[];
  feedback: number[][];
  result: "in_progress" | "won" | "lost";
  started_at: string;
  finished_at?: string;
}

export interface GameStats {
  player_id: string;
  total_games: number;
  total_wins: number;
  current_streak: number;
  best_streak: number;
  avg_guesses: number;
  guess_distribution: number[]; // [count for 1-6 guesses]
  last_game_date?: string;
}

export interface GuessHint {
  word: string;
  entropy_reduction: number;
  expected_remaining: number;
  explanation: string;
}

export interface HintAnalysis {
  top_suggestions: GuessHint[];
  current_entropy: number;
  letter_analysis: Record<string, LetterFrequency>;
}

export interface LetterFrequency {
  occurrences_in_remaining: number;
  position_frequencies: number[]; // [count at position 0-4]
}

export interface GameMode {
  id: string;
  name: string;
  description: string;
  icon: string;
}

export interface WordPack {
  id: string;
  name: string;
  language: string;
  words: string[];
  description?: string;
  author?: string;
  version?: string;
}

export interface Achievement {
  id: string;
  name: string;
  description: string;
  icon: string;
  condition: string;
  unlockedAt?: string;
}

export interface ShareableResult {
  gameId: string;
  playerId: string;
  result: "won" | "lost";
  guesses: number;
  date: string;
  mode: string;
  emoji_board: string; // Emoji grid representation
}
