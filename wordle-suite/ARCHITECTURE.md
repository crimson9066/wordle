# Wordle Suite - Architecture Documentation

## Overview

Wordle Suite is a modular, production-ready Wordle implementation built with a clear separation of concerns:

```
React (UI)
    ↓ (Tauri IPC)
Rust Backend (Game Logic)
    ↓ (SQL)
SQLite (Persistence)
```

## High-Level Architecture

### 1. Presentation Layer (React + TypeScript)
- **Framework**: React 18 with TypeScript
- **State Management**: Zustand (lightweight, TypeScript-friendly)
- **Build Tool**: Vite (fast HMR, optimized bundles)
- **Communication**: Tauri IPC (type-safe, async)

**Key Components**:
- `App.tsx` - Main game component with game board
- `store/appStore.ts` - Global state (player, game, stats)
- `api/gameApi.ts` - Tauri command wrappers
- `types/index.ts` - Shared TypeScript interfaces

**Styling**:
- CSS Grid for responsive layout
- CSS variables for theming (light/dark)
- Animations for tile flips and confetti

### 2. Application Layer (Tauri IPC)
- **Bridge**: `src-tauri/src/commands.rs` - Tauri command handlers
- **Protocol**: Async Rust functions exposed to React via `#[tauri::command]`
- **Serialization**: serde for JSON (de)serialization
- **Type Safety**: Matching TypeScript interfaces with Rust structs

**Key Handlers**:
- Game management: `new_game_cmd`, `make_guess_cmd`
- Validation: `validate_guess_cmd`
- Hints: `get_hint_cmd`
- Stats: `get_player_stats_cmd`

### 3. Business Logic Layer (Rust)

#### `engine.rs` - Pure Game Logic
- **`evaluate_guess(guess, answer)`** - Core evaluation algorithm
- Two-pass evaluation (greens first, yellows second)
- Handles duplicate letters correctly
- No side effects (pure function)
- Heavily unit tested

**Algorithm**:
```
Pass 1: Mark correct positions, decrement letter counts
Pass 2: Mark present positions using remaining counts
```

#### `word_list.rs` - Word Management
- **`DailyWordConfig`** - Manages word list and daily selection
- **`get_daily_word(year, month, day)`** - SHA256-based indexing
- **`is_valid_guess(word)`** - O(1) validation via HashSet
- Deterministic scheduling (same seed = same schedule across installs)

**Daily Word Algorithm**:
```
index = SHA256(YYYY-MM-DD + seed).slice(0,16) as u64 % word_list.len()
```

#### `hint_engine.rs` - Solver & Hints
- **`HintEngine`** - Entropy-based hint generation
- **`analyze(remaining_words, previous_guesses)`** - Comprehensive analysis
- **Information Gain Calculation**: Minimizes expected remaining possibilities
- **Ranking**: Top 5 suggestions by entropy reduction

**Key Metrics**:
- `current_entropy` - Bits of uncertainty in current state
- `entropy_reduction` - How much a guess reduces uncertainty
- `expected_remaining` - Average answers after guess
- `letter_analysis` - Position-specific letter frequencies

#### `db.rs` - Data Persistence
- **SQLite** - Bundled, no external dependency
- **Type-Safe ORM-like wrapper** - Rust structs map to SQL
- **Migrations** - Run automatically on startup
- **CRUD operations** - Players, GameStates, Stats, EventLog

**Tables**:
- `players` - User profiles and settings
- `game_states` - Individual game records
- `game_stats` - Aggregated statistics
- `event_log` - Audit trail and analytics

### 4. Data Layer (SQLite)

**Schema**:

```sql
-- Players: User accounts and preferences
CREATE TABLE players (
  id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  created_at TEXT NOT NULL,
  settings TEXT NOT NULL,    -- JSON
  installs_seed TEXT NOT NULL
);

-- Game States: Individual game records
CREATE TABLE game_states (
  id TEXT PRIMARY KEY,
  player_id TEXT NOT NULL REFERENCES players(id),
  game_mode TEXT NOT NULL,
  word_id TEXT NOT NULL,
  guesses TEXT NOT NULL,     -- JSON array
  feedback TEXT NOT NULL,    -- JSON array of arrays [0=absent, 1=present, 2=correct]
  result TEXT NOT NULL,      -- "in_progress", "won", "lost"
  started_at TEXT NOT NULL,
  finished_at TEXT
);

-- Game Stats: Aggregated player statistics
CREATE TABLE game_stats (
  player_id TEXT PRIMARY KEY REFERENCES players(id),
  total_games INTEGER NOT NULL,
  total_wins INTEGER NOT NULL,
  current_streak INTEGER NOT NULL,
  best_streak INTEGER NOT NULL,
  avg_guesses REAL NOT NULL,
  guess_distribution TEXT NOT NULL,  -- JSON [count_1, count_2, ..., count_6]
  last_game_date TEXT
);

-- Event Log: Audit trail and analytics
CREATE TABLE event_log (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id TEXT NOT NULL REFERENCES players(id),
  event_type TEXT NOT NULL,
  event_data TEXT NOT NULL,  -- JSON
  timestamp TEXT NOT NULL
);
```

## Data Flow

### Game Start Flow
```
React (new_game_cmd)
    → Tauri IPC
    → cmd_new_game (commands.rs)
    → Database (create game_state entry)
    → DailyWordConfig (get_daily_word)
    → Return GameState
    → React (store currentGame)
```

### Guess Submission Flow
```
React User Input
    → gameApi.makeGuess(gameId, guess)
    → Tauri IPC (make_guess_cmd)
    → Validate guess (word_list.rs)
    → Evaluate guess (engine.rs)
    → Update game state (db.rs)
    → Update stats (db.rs)
    → Return feedback
    → React (update UI, refresh state)
```

### Hint Generation Flow
```
React (get_hint_cmd)
    → Tauri IPC
    → HintEngine::analyze()
    → For each candidate word:
        → For each remaining answer:
            → evaluate_guess()
        → Calculate partition entropy
        → Compute entropy reduction
    → Sort by entropy reduction
    → Return top 5 suggestions
    → React (display hints)
```

## Module Dependencies

```
main.rs (entry point)
├── commands.rs (IPC handlers)
│   ├── engine.rs (game logic)
│   ├── word_list.rs (word management)
│   ├── hint_engine.rs (solver)
│   └── db.rs (persistence)
├── db.rs
│   └── (rusqlite, chrono, serde_json)
└── (tauri, tokio)
```

## Design Patterns

### 1. Pure Functions (engine.rs)
- `evaluate_guess()` - No side effects, deterministic
- Enables testing, reasoning, and reuse
- Can be optimized independently

### 2. Trait Objects & Generics (hint_engine.rs)
- `HintEngine::new()` - Takes word list, creates self-contained solver
- Can be used independently or as Tauri handler

### 3. State Machine (game state)
- Game states: `in_progress` → `won` or `lost`
- Prevents invalid operations (guess after game ends)

### 4. Type-Safe Serialization
- Rust structs with `#[derive(Serialize, Deserialize)]`
- TypeScript interfaces matching Rust structs
- Compile-time checking on both sides

### 5. Lazy Initialization
- Database: Created on first `init_db_cmd` call
- DailyWordConfig: Static word list, initialized at startup
- App state: Managed by Tauri State container

## Performance Considerations

### Game Evaluation
- Time: O(1) per guess (5 characters, constant size)
- Space: O(k) where k = 26 letters
- Optimization: Precompute letter counts once per guess

### Hint Generation
- Naive: O(n²) - evaluate each candidate against all remaining answers
- Optimized: O(c·n) - evaluate top ~1000 candidate guesses against n remaining answers
- Cache: Results per game state (could be extended)

### Word Validation
- Naive: O(n) - linear search through word list
- Optimized: O(1) - HashSet lookup
- Implementation: `DailyWordConfig::get_valid_set()`

### Database Queries
- Indexed on: `player_id`, `game_id` for fast lookups
- Single trip: Get/save game state in one query
- Migrations: Schema version controlled, auto-upgrade

### React Rendering
- Memoization: Game board tiles could use `React.memo()`
- Lazy load: Stats sidebar loads after game renders
- Virtual scroll: Not needed (max 6 guesses visible)

## Security Architecture

### No Auth (Local App)
- Each install gets unique `installs_seed`
- Used for scheduling consistency, not authentication
- Player profile is local, not synced

### Encryption (Optional)
- Database: Could be encrypted with password (future)
- Backups: Encrypted export with user password (future)
- Sync: TLS for any network communication (future)

### Sandboxing (Future)
- Plugin execution: Run in isolated context
- Word pack integrity: Checksum/signature verification
- Plugin permissions: Restrict file/network access

### Privacy by Design
- No telemetry by default
- Event log is local only
- Export on demand (opt-in)
- No external API calls unless explicitly enabled

## Testing Strategy

### Unit Tests (Pure Functions)
- Location: `#[cfg(test)]` modules in each .rs file
- Coverage: engine.rs (100%), word_list.rs (95%), db.rs (90%)
- Tools: `cargo test`

### Integration Tests (Module Interaction)
- Location: `src-tauri/tests/` (future)
- Scope: Game flow (new → guess → win/loss)
- Tools: `cargo test --test integration_tests`

### UI Tests (React Components)
- Location: `src/**/*.test.tsx` (future)
- Framework: Vitest + React Testing Library
- Tools: `npm test`

### E2E Tests (Full App)
- Location: `.github/workflows/e2e.yml` (future)
- Framework: Tauri testing utilities / Playwright
- Scope: Launch app → play game → verify results

### Performance Tests
- Location: `src-tauri/benches/` (future)
- Benchmark: evaluate_guess, hint generation
- Tools: `cargo bench`

## Extensibility

### Adding a Game Mode
1. Create `src-tauri/src/modes/classic.rs`
2. Implement `GameMode` trait
3. Return from `get_game_mode()` handler
4. React component updates game rules

### Adding Hint Type
1. Add variant to `HintType` enum in `hint_engine.rs`
2. Implement `generate_hint_*()` function
3. Add to `HintAnalysis` response
4. React displays new hint type

### Adding Database Table
1. Add to `db.rs::run_migrations()`
2. Implement CRUD in `impl Database`
3. Expose via Tauri command
4. Call from React

### Adding Persistence
1. Extend `GameState` struct
2. Update `save_game_state()` to serialize new fields
3. Update `get_game_state()` to deserialize
4. Update TypeScript `GameState` interface

## Deployment Architecture

### Binary Distribution
- Platform-specific installers (Windows .msi, Linux .AppImage)
- Code signed with developer certificate (optional)
- Checksums for integrity verification

### Update Mechanism
- Manual: Check website, download new version
- Auto: Built-in updater (future, using tauri-updater)
- Seamless: Update in background, prompt on restart

### Database Migration
- Automatic on app start
- Backward compatible (old schema→new schema)
- Version tracking in schema

## Monitoring & Observability

### Event Log
- Local audit trail of user actions
- Opt-in remote upload (future)
- Can be exported for analysis

### Debug Mode (Future)
- Enable via settings or env var
- Show internal state: word counts, entropy, partition sizes
- Log all IPC calls and responses

### Performance Monitoring (Future)
- Time hints API
- Track frequency of game modes
- Identify slow queries

---

**Architecture Summary**: Clean separation between UI (React), business logic (Rust), and data (SQLite), connected via async IPC. Each layer is independently testable and deployable. Security and privacy built in from day one.
