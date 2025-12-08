# Wordle Suite Development Guide

## Overview
This is a comprehensive Wordle game suite built with:
- **Backend**: Tauri + Rust (game engine, solver, database)
- **Frontend**: React + TypeScript (UI, player experience)
- **Database**: SQLite with migrations
- **Build**: Vite for frontend, Cargo for Rust

## Quick Start

### Prerequisites
- Rust 1.70+ (https://rustup.rs/)
- Node.js 18+ (https://nodejs.org/)
- npm or yarn

### Setup
```bash
# Install dependencies
npm install

# Dev mode (hot reload)
npm run tauri-dev

# Build for production
npm run tauri-build

# Run tests
npm test
```

## Project Structure

```
wordle-suite/
├── src-tauri/              Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs         App entry, IPC handlers
│   │   ├── engine.rs       Game logic (evaluateGuess)
│   │   ├── word_list.rs    Daily words, validation
│   │   ├── hint_engine.rs  Solver with entropy
│   │   ├── db.rs           SQLite persistence
│   │   └── commands.rs     Tauri IPC handlers
│   └── Cargo.toml
├── src/                    React frontend
│   ├── App.tsx             Main component
│   ├── main.tsx            Entry point
│   ├── types/              TypeScript types
│   ├── store/              Zustand state management
│   └── api/                Tauri API client
├── index.html
├── package.json
├── vite.config.ts
└── README.md
```

## Core Architecture Highlights

### 1. Game Engine (`src-tauri/src/engine.rs`)
- **Pure function**: `evaluate_guess(guess, answer) -> GuessEvaluation`
- **Two-pass algorithm**: Greens first, then yellows respecting available counts
- **Handles duplicates correctly**: e.g., SPEED vs ERASE
- **Heavily tested**: Unit tests for all edge cases

### 2. Daily Word Selection (`src-tauri/src/word_list.rs`)
- **Deterministic**: SHA256(date + seed) % word_list.len()
- **No server needed**: All players get same word for the date
- **Reproducible**: Seed in config ensures consistency across reinstalls
- **O(1) validation**: HashSet for instant word lookup

### 3. Hint Engine (`src-tauri/src/hint_engine.rs`)
- **Entropy-based**: Information-gain calculation
- **Top 5 suggestions**: Ranked by entropy reduction
- **Educational**: Explains why each guess is optimal
- **Cached evaluation**: Evaluates against smaller candidate set for performance

### 4. Persistence (`src-tauri/src/db.rs`)
- **SQLite**: Bundled, no external setup
- **Migrations**: Auto-run on startup
- **Players, Games, Stats, Events**: Full data model
- **Privacy-first**: Local storage, opt-in sync only

### 5. IPC Bridge (`src-tauri/src/commands.rs`, `src/api/gameApi.ts`)
- **Type-safe**: serde on Rust, TypeScript interfaces on React
- **Async/await**: Both sides handle async naturally
- **Error handling**: Graceful error propagation

## Development Workflow

### Adding a Feature

1. **Backend (Rust)**:
   - Add function to `engine.rs` or appropriate module
   - Write unit tests
   - Add Tauri command in `commands.rs`
   - Add command handler in `main.rs`

2. **Frontend (React)**:
   - Add API method in `src/api/gameApi.ts`
   - Add types in `src/types/index.ts` if needed
   - Create component or hook
   - Update Zustand store if global state needed
   - Add styles to `index.css` or component CSS

### Testing

**Rust tests**:
```bash
cd src-tauri
cargo test              # Run all tests
cargo test engine::     # Run specific module
cargo test --lib       # Run library tests only
```

**React tests**:
```bash
npm test                # Run Jest/Vitest
npm run test:coverage   # With coverage report
```

### Building

**Development**:
```bash
npm run tauri-dev       # Hot reload, debug builds
```

**Production**:
```bash
npm run tauri-build     # Optimized, signed binaries
```

## Key Algorithms

### Guess Evaluation (Two-Pass)
```
Pass 1: Mark all correct positions (green), decrement letter counts
Pass 2: Mark present letters (yellow) using remaining counts

Example: Guess SPEED vs Answer ERASE
- S: position 0 = green, decrement S count
- P: position 1 ≠ answer[1], check remaining... P in answer @ pos 2 = yellow
- E: position 2 ≠ answer[2], but only 1 E in ERASE (@ pos 0, already used) = absent
- E: position 3 ≠ answer[3], no E left = absent  
- D: position 4 = green, exact match
```

### Entropy-Based Hints
```
For each candidate guess g:
  Partition remaining answers by feedback pattern of g
  entropy_after = Shannon entropy of partition distribution
  entropy_reduction = entropy_before - entropy_after

Select guesses with highest entropy_reduction
(minimizes expected remaining possibilities)
```

### Deterministic Daily Words
```
index = SHA256(YYYY-MM-DD + "wordle-suite-v1") mod word_list.len()

Same for all players, same device/install, fully deterministic
Seed in config allows reproducible schedules
```

## Database Schema

### Players
```sql
CREATE TABLE players (
  id TEXT PRIMARY KEY,
  display_name TEXT,
  created_at TEXT,
  settings TEXT,      -- JSON
  installs_seed TEXT
);
```

### Game States
```sql
CREATE TABLE game_states (
  id TEXT PRIMARY KEY,
  player_id TEXT REFERENCES players(id),
  game_mode TEXT,
  word_id TEXT,
  guesses TEXT,       -- JSON array
  feedback TEXT,      -- JSON array of arrays
  result TEXT,        -- "in_progress", "won", "lost"
  started_at TEXT,
  finished_at TEXT
);
```

### Stats
```sql
CREATE TABLE game_stats (
  player_id TEXT PRIMARY KEY REFERENCES players(id),
  total_games INTEGER,
  total_wins INTEGER,
  current_streak INTEGER,
  best_streak INTEGER,
  avg_guesses REAL,
  guess_distribution TEXT,  -- JSON [1-guess, 2-guess, ...]
  last_game_date TEXT
);
```

### Event Log (Analytics)
```sql
CREATE TABLE event_log (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id TEXT REFERENCES players(id),
  event_type TEXT,
  event_data TEXT,    -- JSON
  timestamp TEXT
);
```

## Tauri IPC Commands

All commands are in `main.rs` and called via `invoke()` from React:

```typescript
// Example: Make a guess
const [response, error] = await invoke("make_guess_cmd", {
  request: { game_id: "...", guess: "STALE" }
});
```

Available commands:
- `eval_guess_cmd` - Evaluate guess
- `get_daily_word_cmd` - Get daily word
- `validate_guess_cmd` - Check if word valid
- `new_game_cmd` - Start new game
- `make_guess_cmd` - Submit guess
- `get_hint_cmd` - Get hint suggestions
- `get_or_create_player_cmd` - Player profile
- `get_player_stats_cmd` - Player stats
- `init_db_cmd` - Initialize database

## Performance Considerations

1. **Hint engine**: Evaluates against top 1000 words only (not full 500k+)
2. **Word validation**: O(1) using HashSet
3. **Database**: Indexed on player_id, game_id for fast queries
4. **Frontend**: React.memo for game board tiles, lazy load stats sidebar

## Security & Privacy

✅ **By default**:
- No telemetry
- No network requests
- All data local SQLite
- No authentication needed

🔒 **Optional**:
- Encrypted backups (AES with password)
- TLS for opt-in cloud sync
- Integrity checks for word packs
- Sandboxed plugin execution (future)

## Troubleshooting

### Build fails with "cargo not found"
- Install Rust from https://rustup.rs/
- Run `rustup target add x86_64-pc-windows-msvc` (Windows)

### Tauri app won't launch
- Check `.vscode/launch.json` for debug configuration
- Run `npm run tauri-dev` instead of debugging
- Check app data dir: `%APPDATA%\wordle-suite` (Windows)

### Database errors
- Delete `wordle.db` and `wordle.db-shm` to reset
- Check permissions on app data directory
- Verify SQLite version via `rusqlite` features

### TypeScript errors in IDE
- Run `npm install` to get all dependencies
- Restart TypeScript server (Cmd+Shift+P → "Restart TS Server")
- Check paths in `tsconfig.json` match actual files

## Next Steps

1. **Install dependencies**: `npm install`
2. **Run dev server**: `npm run tauri-dev`
3. **Run tests**: `npm test`
4. **Read the code**: Start with `src-tauri/src/engine.rs`
5. **Explore the UI**: `src/App.tsx`

Enjoy building! 🚀
