# Wordle Suite - Module Reference

## Overview

This document provides a quick reference for each module in the Wordle Suite project.

---

## Backend Modules (Rust)

###  `src-tauri/src/engine.rs` - Core Game Logic
**Purpose**: Pure function for evaluating Wordle guesses

**Key Function**:
- `evaluate_guess(guess: &str, answer: &str) -> GuessEvaluation`

**Algorithm**:
```
Two-pass evaluation:
1. Pass 1: Mark correct positions (green), decrement letter counts
2. Pass 2: Mark present letters (yellow) using remaining counts
```

**Why This Works**:
Handles duplicate letters correctly. Example:
- Guess: SPEED, Answer: ERASE
- S: correct (green)
- P: in word but wrong position (yellow)
- E: duplicate, already used (absent)
- E: no more E's available (absent)
- D: correct (green)

**Tests**: 5 unit tests covering:
- Exact matches
- No matches
- Present letters
- Duplicate handling
- Case insensitivity

**Dependencies**: None (pure Rust stdlib)

**File Size**: ~100 lines

---

###  `src-tauri/src/word_list.rs` - Word Management
**Purpose**: Manage word lists and daily word selection

**Main Type**:
- `DailyWordConfig` struct with methods

**Key Methods**:
- `new()` - Create with default word list
- `get_daily_word(year, month, day)` - Get word for date (SHA256-based)
- `is_valid_guess(word)` - Check if word in list (O(1))
- `get_valid_set()` - Return HashSet for validation

**Daily Word Algorithm**:
```
index = SHA256(YYYY-MM-DD + seed).slice(0,16) as u64 % word_list.len()

Example:
date = "2025-12-08"
seed = "wordle-suite-v1"
combined = "2025-12-08wordle-suite-v1"
hash = SHA256(combined)
index = hash as u64 % 500
word = WORD_LIST[index]
```

**Why Deterministic**:
- Same date → same word for all players
- No server needed
- Reproducible across installs (seed in config)
- Non-obvious schedule (cryptographic hash)

**Word List**: 500+ words by default (customizable)

**Tests**: 4 unit tests covering:
- Deterministic output (same input → same output)
- Different dates → different words
- Valid word checking
- Set lookup

**Dependencies**: sha2, hex, lazy_static

**File Size**: ~200 lines

---

###  `src-tauri/src/hint_engine.rs` - Solver & Hints
**Purpose**: Generate optimal hints using entropy-based calculation

**Main Type**:
- `HintEngine` struct

**Key Method**:
- `analyze(remaining_words, previous_guesses) -> HintAnalysis`

**Algorithm**:
```
For each candidate guess:
  For each possible answer in remaining_words:
    pattern = evaluate_guess(candidate, answer)
    count patterns
  
  entropy_after = Shannon entropy of pattern distribution
  entropy_reduction = entropy_before - entropy_after
  expected_remaining = average answers after guess

Sort by entropy_reduction (descending)
Return top 5
```

**Why This Works**:
- Minimizes expected remaining answers
- Explains why each guess is good
- Educational (information theory in action)
- Can be optimized (evaluates top 1000 candidates only)

**Output**:
```
HintAnalysis {
  top_suggestions: [GuessHint],     // Top 5 suggestions
  current_entropy: f64,             // Bits of uncertainty
  letter_analysis: HashMap,         // Position-specific frequencies
}

GuessHint {
  word: String,
  entropy_reduction: f64,           // How much this guess reduces uncertainty
  expected_remaining: f64,          // Average answers left
  explanation: String,              // Human-readable reason
}
```

**Performance**:
- O(c × n) where c=candidates (~1000), n=remaining answers
- Caching potential between turns
- Evaluated on smaller candidate set for speed

**Tests**: 2 unit tests covering:
- Entropy calculation
- Basic hint generation

**Dependencies**: crate::engine (depends on)

**File Size**: ~300 lines

---

###  `src-tauri/src/db.rs` - SQLite Persistence
**Purpose**: Store and retrieve player profiles, game states, and statistics

**Main Type**:
- `Database` struct with methods
- Associated types: `PlayerProfile`, `GameState`, `GameStats`

**Key Methods**:
- `open(path)` - Open/create database
- `run_migrations()` - Create tables (auto on open)
- `create_player()` - Insert new player
- `get_player()` - Retrieve player profile
- `save_game_state()` - Insert/update game
- `get_game_state()` - Retrieve game
- `get_or_create_stats()` - Get or create stats
- `update_stats_after_game()` - Update after win/loss
- `log_event()` - Log action for analytics

**Database Schema**:

```sql
players {
  id TEXT PRIMARY KEY,
  display_name TEXT,
  created_at TEXT,
  settings TEXT,           -- JSON
  installs_seed TEXT
}

game_states {
  id TEXT PRIMARY KEY,
  player_id TEXT REFERENCES players,
  game_mode TEXT,          -- "Classic", "Timed", etc.
  word_id TEXT,            -- The answer
  guesses TEXT,            -- JSON array
  feedback TEXT,           -- JSON [[0,1,2,...], ...]
  result TEXT,             -- "in_progress", "won", "lost"
  started_at TEXT,
  finished_at TEXT
}

game_stats {
  player_id TEXT PRIMARY KEY REFERENCES players,
  total_games INTEGER,
  total_wins INTEGER,
  current_streak INTEGER,
  best_streak INTEGER,
  avg_guesses REAL,
  guess_distribution TEXT, -- JSON [count_1, count_2, ...]
  last_game_date TEXT
}

event_log {
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id TEXT REFERENCES players,
  event_type TEXT,         -- "game_started", "game_won", etc.
  event_data TEXT,         -- JSON
  timestamp TEXT
}
```

**Why This Design**:
- Separate stats table for fast aggregation
- JSON for flexible storage (settings, distribution)
- Event log for future analytics (opt-in)
- Migrations ensure schema upgrades

**Error Handling**:
- Custom `DbError` type
- `DbResult<T> = Result<T, DbError>`
- Converts rusqlite errors

**Tests**: 2 unit tests covering:
- Database creation
- Player management
- Stats updates

**Dependencies**: rusqlite, chrono, serde, serde_json

**File Size**: ~400 lines

---

###  `src-tauri/src/commands.rs` - Tauri IPC Handlers
**Purpose**: Bridge between React frontend and Rust backend

**Key Functions** (exposed as `#[tauri::command]`):
- `cmd_evaluate_guess()` - Evaluate a guess
- `cmd_get_daily_word()` - Get daily word
- `cmd_validate_guess()` - Check if word valid
- `cmd_new_game()` - Start new game
- `cmd_make_guess()` - Submit guess
- `cmd_get_hint()` - Get hints
- `cmd_get_or_create_player()` - Player management
- `cmd_get_player_stats()` - Get statistics

**Request/Response Pattern**:
```rust
// Request type (matches TypeScript interface)
pub struct GuessRequest {
    pub game_id: String,
    pub guess: String,
}

// Handler converts to Tauri command
#[tauri::command]
fn make_guess_cmd(request: GuessRequest, ...) -> Result<..., String>
```

**Serialization**:
- Uses serde for JSON conversion
- Matches TypeScript interface definitions
- Type-safe round-trip

**Error Handling**:
- Returns `Result<T, String>` for error propagation
- JavaScript receives error in catch block

**No Side Effects**:
- Handlers are thin wrappers
- Actual logic in other modules
- Just coordinate and serialize

**Dependencies**: serde, serde_json, uuid, chrono

**File Size**: ~200 lines

---

###  `src-tauri/src/main.rs` - App Entry Point
**Purpose**: Initialize Tauri app, setup IPC, manage state

**Key Components**:
- `AppState` struct - Global state holder
- `run()` function - Main app initialization
- IPC command handlers (thin wrappers)
- System tray event handler

**State Management**:
```rust
pub struct AppState {
    pub db: Mutex<Option<Database>>,
    pub daily_word_config: DailyWordConfig,
}
```

**Why Mutex<Option>**:
- Database created lazily on first use
- Thread-safe access
- Option for initialization error handling

**System Tray**:
- Show/Hide window
- Quit application
- Menu items in system tray

**Command Registration**:
All commands listed with `#[tauri::command]` macro
```rust
.invoke_handler(tauri::generate_handler![
    eval_guess_cmd,
    validate_guess_cmd,
    new_game_cmd,
    // ...
])
```

**File Size**: ~150 lines

---

## Frontend Modules (React/TypeScript)

###  `src/App.tsx` - Main Game Component
**Purpose**: Render game board and manage game flow

**Key Sections**:
1. **Game Board** - 6×5 tile grid
2. **Input Area** - Text input + submit button
3. **Stats Sidebar** - Player statistics
4. **Header** - Title + theme toggle
5. **Footer** - Project info

**State Management**:
- `gameState` - Current game data
- `currentGuess` - User input buffer
- `feedback` - Array of feedback arrays
- `playerStats` - Player statistics
- `message` - Display messages
- `theme` - Current theme

**Key Functions**:
- `initializeApp()` - Setup on mount
- `handleKeyPress()` - Keyboard input
- `submitGuess()` - Send guess to backend
- `toggleTheme()` - Switch theme

**Hooks Used**:
- `useState` - Local component state
- `useEffect` - Initialize on mount, run side effects
- `invoke` - Tauri IPC calls

**Dependencies**: React, @tauri-apps/api

**File Size**: ~150 lines

---

###  `src/store/appStore.ts` - State Management
**Purpose**: Centralized global state with Zustand

**State Structure**:
```typescript
interface AppStore {
  // Data
  player: PlayerProfile | null
  currentGame: GameState | null
  stats: GameStats | null
  
  // UI
  theme: "light" | "dark"
  showHints: boolean
  showSettings: boolean
  message: string | null
  messageType: "success" | "error" | "info" | "warning"
  
  // Actions
  setPlayer, setCurrentGame, setStats,
  setTheme, setShowHints, setShowSettings,
  setMessage, initializeApp, updateSettings
}
```

**Why Zustand**:
- Minimal boilerplate
- TypeScript-friendly
- No provider wrapper needed
- Small bundle size

**Persistence**:
- Theme saved to localStorage
- Survives app refresh

**Async Initialization**:
```typescript
initializeApp: async (playerName) => {
  // Initialize database
  // Create/get player
  // Create new game
  // Load stats
}
```

**File Size**: ~150 lines

---

###  `src/api/gameApi.ts` - Tauri API Client
**Purpose**: Type-safe wrapper around Tauri commands

**Key Methods**:
- `gameApi.evaluateGuess()` - Evaluate guess
- `gameApi.getDailyWord()` - Get word for date
- `gameApi.validateGuess()` - Check if valid
- `gameApi.newGame()` - Start game
- `gameApi.makeGuess()` - Submit guess
- `gameApi.getHint()` - Get hints
- `gameApi.getOrCreatePlayer()` - Player profile
- `gameApi.getPlayerStats()` - Statistics
- `gameApi.initDatabase()` - Init DB

**Why Wrapper**:
- Single source of truth for command names
- Type-safe parameters and returns
- Error handling in one place
- Easy to update if backend changes

**Error Utility**:
```typescript
handleApiError(error: unknown): string
```

**Pattern**:
```typescript
// Wrapper
async evaluateGuess(guess: string, answer: string): Promise<GuessEvaluation> {
  return invoke("eval_guess_cmd", { request: { guess, answer } });
}

// Usage
const result = await gameApi.evaluateGuess("SLATE", "STALE");
```

**File Size**: ~100 lines

---

###  `src/types/index.ts` - TypeScript Type Definitions
**Purpose**: Centralized types matching Rust structs

**Key Types**:
- `LetterFeedback` - Enum: Absent, Present, Correct
- `GuessEvaluation` - Result of evaluation
- `PlayerProfile` - User account
- `PlayerSettings` - User preferences
- `GameState` - Current game data
- `GameStats` - Aggregated statistics
- `GuessHint` - Suggestion with explanation
- `HintAnalysis` - Analysis result

**Serialization Compatibility**:
- Types match Rust struct serde output
- Enums map to discriminants
- JSON serializable

**Usage Pattern**:
```typescript
// In Rust
#[derive(Serialize)]
pub struct GameState { ... }

// In TypeScript
export interface GameState { ... }
```

**File Size**: ~150 lines

---

###  `src/index.css` - Global Styles
**Purpose**: Styling for entire application

**Key Sections**:
1. **CSS Variables** - Theme colors
2. **Base Styles** - Reset, body, typography
3. **.app Layout** - Main container (flex column)
4. **Game Board** - Grid layout
5. **Tiles** - Feedback styling
6. **Animations** - Flip, slide, shake
7. **Dark Mode** - Color variable overrides
8. **Responsive** - Mobile breakpoints

**Theming Strategy**:
```css
:root {
  --primary-color: #6aaa64;
  --secondary-color: #b59f3b;
  --danger-color: #787c7e;
  --bg-color: #ffffff;
  --text-color: #000000;
}

.dark-theme {
  --primary-color: #6aaa64;
  --bg-color: #121213;
  --text-color: #ffffff;
}
```

**Animations**:
```css
@keyframes flip {
  0% { transform: rotateX(0); }
  50% { transform: rotateX(90deg); }
  100% { transform: rotateX(0); }
}
```

**File Size**: ~400+ lines

---

## Configuration Files

###  `Cargo.toml` - Rust Workspace
**Purpose**: Workspace configuration and shared dependencies

**Workspace Members**:
- `src-tauri` - Main application

**Workspace-Level Configuration**:
- Version
- Edition
- Authors
- License
- Lint rules

**Dependencies** (in src-tauri/):
- tauri - Desktop framework
- serde - Serialization
- rusqlite - Database
- sha2 - Hashing
- uuid - ID generation
- chrono - Date/time

---

###  `package.json` - NPM Configuration
**Purpose**: Frontend dependencies and build scripts

**Key Scripts**:
- `tauri-dev` - Dev mode
- `tauri-build` - Production build
- `test` - Run tests
- `lint` - Run ESLint
- `type-check` - TypeScript check

**Dependencies**:
- react - UI framework
- @tauri-apps/api - Frontend API
- zustand - State management
- framer-motion - Animations (optional)
- lucide-react - Icons (optional)

---

###  `vite.config.ts` - Frontend Build
**Purpose**: Vite configuration for React

**Key Settings**:
- React plugin enabled
- Proper JSX handling
- Dev port: 5173
- Vite environment prefix (VITE_)

---

###  `tauri.conf.json` - Tauri App Config
**Purpose**: App window, bundle, and security settings

**Key Settings**:
- Window size: 1200×900 (resizable)
- Minimum size: 800×600
- Bundle configuration
- Security settings

---

###  `.github/workflows/ci-cd.yml` - GitHub Actions
**Purpose**: Automated testing and building

**Matrix Strategy**:
- OS: Windows, Linux, macOS
- Node: 18, 20
- Rust: stable

**Jobs**:
1. **test** - Run linters and tests
2. **build-artifacts** - Create installers
3. **security-audit** - Check dependencies

---

## Summary Table

| File | Lines | Purpose | Language |
|------|-------|---------|----------|
| engine.rs | 100 | Game logic | Rust |
| word_list.rs | 200 | Word management | Rust |
| hint_engine.rs | 300 | Solver | Rust |
| db.rs | 400 | Persistence | Rust |
| commands.rs | 200 | IPC handlers | Rust |
| main.rs | 150 | App entry | Rust |
| App.tsx | 150 | Game UI | React/TS |
| appStore.ts | 150 | State mgmt | TS |
| gameApi.ts | 100 | API client | TS |
| types/index.ts | 150 | Types | TS |
| index.css | 400+ | Styles | CSS |
| App.css | 100+ | Component styles | CSS |
| **Total** | **2,300+** | **Full app** | **Rust/TS** |

---

**Each module is focused, testable, and well-documented.**
