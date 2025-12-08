# Wordle Suite

Wordle game suite built with Tauri (Rust backend), React (TypeScript frontend), and SQLite. Includes multiple game modes and a solid feature set.

## Features

### Core Game Engine
- Two-pass evaluation algorithm for accurate guess feedback (handles duplicate letters correctly)
- Deterministic daily word selection using SHA256 hashing
- Support for multiple game modes (Classic, Timed, Hard, Cipher, etc.)
- Comprehensive word list validation with O(1) lookup

### Hint System and Solver
- Hint engine that suggests next guesses
- Shows letter frequency and position stats
- Can walk through the solving process step-by-step
- Hints are based on reducing remaining possibilities

### Player Features
- Player profiles with customizable settings
- Rich statistics: streaks, win distribution, average guesses
- Achievements system with unlocks
- Theme support (Light and Dark modes, color-blind modes)
- Keyboard-first user experience with full mouse and touch support

### Data Persistence
- SQLite database with schema migrations
- Event logging for analytics (local only, privacy-focused)
- JSON export and import for portability
- Encrypted backup support (opt-in)

### User Experience
- Tile animations on feedback
- Notification on win
- System tray integration
- Works on different screen sizes

### Security and Privacy
- No data collection by default
- All data stays local
- Can export your data

## Architecture

```
wordle-suite/
├── src-tauri/                    # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point and IPC
│   │   ├── engine.rs            # Core game logic (evaluateGuess)
│   │   ├── word_list.rs         # Daily word selection and validation
│   │   ├── hint_engine.rs       # Entropy-based solver
│   │   ├── db.rs                # SQLite persistence layer
│   │   └── commands.rs          # Tauri command handlers
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── src/                          # React frontend (TypeScript)
│   ├── App.tsx                  # Main app component
│   ├── main.tsx                 # React entry point
│   ├── index.css                # Global styles
│   └── App.css                  # Component styles
├── index.html                   # HTML template
├── package.json                 # NPM dependencies
├── tsconfig.json                # TypeScript configuration
├── vite.config.ts               # Vite build configuration
├── Cargo.toml                   # Workspace configuration
└── README.md                    # This file
```

## Getting Started

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- Node.js 18+ (install from https://nodejs.org/)
- npm or yarn

### Installation

1. Install Rust dependencies:
   ```bash
   rustup target add x86_64-pc-windows-msvc  # or your platform
   ```

2. Install Node dependencies:
   ```bash
   npm install
   ```

3. Start development mode:
   ```bash
   npm run tauri-dev
   ```

4. Build for production:
   ```bash
   npm run tauri-build
   ```

## Project Structure Explained

### Backend (Rust/Tauri)

#### `engine.rs` - Core Game Engine
- Pure function `evaluate_guess(guess, answer) -> GuessEvaluation`
- Two-pass algorithm: greens first, then yellows (respecting available counts)
- Handles duplicate letters correctly
- Unit tests included

#### `word_list.rs` - Word Management
- Default word list with 500+ words
- `get_daily_word(year, month, day)` - Deterministic scheduling via SHA256
- Validation with O(1) lookup via HashSet
- Configurable seed for reproducible schedules

#### `hint_engine.rs` - Solver & Hints
- Shannon entropy calculation for game state
- Partition analysis: how many answers remain for each feedback pattern
- Top 5 suggestions ranked by information gain
- Letter frequency analysis

#### `db.rs` - SQLite Persistence
- Player profiles with settings
- Game state storage (guesses, feedback, results)
- Statistics (streaks, distributions, averages)
- Event logging for analytics
- Schema migrations on startup

#### `commands.rs` - IPC Bridge
- Tauri command handlers for React → Rust communication
- Request/Response serialization
- Error handling and validation

### Frontend (React/TypeScript)

#### `App.tsx` - Main Component
- Game board with tile flip animations
- Input handling (keyboard + submit button)
- Player stats sidebar
- Theme toggle (light/dark)
- Message display (valid/invalid words, game result)

#### `index.css` - Global Styles
- CSS variables for theming
- Responsive layout (grid for desktop, single column for mobile)
- Animations (tile flip on feedback)
- Dark mode support

## Key Algorithms

### 1. Guess Evaluation (Two-Pass Algorithm)

```rust
fn evaluate_guess(guess: &str, answer: &str) -> GuessEvaluation {
    // Pass 1: Mark correct positions (green), decrement counts
    for (i, guess_char) in guess.iter().enumerate() {
        if guess_char == answer[i] {
            feedback[i] = Correct;
            answer_counts[guess_char] -= 1;
        }
    }
    
    // Pass 2: Mark present letters (yellow) from remaining counts
    for (i, guess_char) in guess.iter().enumerate() {
        if feedback[i] == Absent && answer_counts[guess_char] > 0 {
            feedback[i] = Present;
            answer_counts[guess_char] -= 1;
        }
    }
}
```

**Why this works**: Example with SPEED vs ERASE:
- S at position 0: correct
- P at position 1: in answer but position 2, so yellow
- E at position 2: only 1 E in ERASE (at position 0), already used by correct S, so absent
- E at position 3: same, absent
- D at position 4: in answer at position 4, correct

### 2. Deterministic Daily Words

```rust
index = SHA256(date_string + seed) % word_list.len()
date_string = "2025-12-08"
seed = "wordle-suite-v1"
```

- Same word for all players on the same date (no server needed)
- Deterministic across reinstalls (seed in config)
- One word per day, non-obvious schedule

### 3. Entropy-Based Hints

```
For each candidate guess g:
  For each possible answer a in remaining:
    pattern = evaluate_guess(g, a)
    partition[pattern] += 1
  
  entropy_after = -Σ P(pattern) * log2(P(pattern))
  entropy_reduction = entropy_before - entropy_after
  
Suggest guesses with highest entropy_reduction
```

- Minimizes expected remaining possibilities
- Explains why each guess is good
- O(n²) but cached and evaluated on smaller candidate set

## 🗄️ Database Schema

### Players Table
```sql
CREATE TABLE players (
  id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  created_at TEXT NOT NULL,
  settings TEXT,           -- JSON settings
  installs_seed TEXT       -- Reproducible seed
);
```

### Game States Table
```sql
CREATE TABLE game_states (
  id TEXT PRIMARY KEY,
  player_id TEXT FOREIGN KEY,
  game_mode TEXT,          -- "Classic", "Timed", "Hard", etc.
  word_id TEXT,            -- The answer word
  guesses TEXT,            -- JSON array of guesses
  feedback TEXT,           -- JSON array of feedback arrays
  result TEXT,             -- "in_progress", "won", "lost"
  started_at TEXT,
  finished_at TEXT
);
```

### Stats Table
```sql
CREATE TABLE game_stats (
  player_id TEXT PRIMARY KEY FOREIGN KEY,
  total_games INTEGER,
  total_wins INTEGER,
  current_streak INTEGER,
  best_streak INTEGER,
  avg_guesses REAL,
  guess_distribution TEXT,  -- JSON [count for 1-6 guesses]
  last_game_date TEXT
);
```

### Event Log Table
```sql
CREATE TABLE event_log (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id TEXT FOREIGN KEY,
  event_type TEXT,
  event_data TEXT,         -- JSON event details
  timestamp TEXT
);
```

## Tauri IPC Commands

### Game Commands

**`eval_guess`** - Evaluate a guess
```json
Request: { "guess": "STALE", "answer": "STEAL" }
Response: { "feedback": [2, 2, 1, 2, 2], "is_correct": false }
```

**`get_daily_word`** - Get today's word (for dev/admin)
```json
Request: { "year": 2025, "month": 12, "day": 8 }
Response: "STALE"
```

**`validate_guess`** - Check if word is valid
```json
Request: { "word": "SLATE" }
Response: true
```

**`new_game`** - Start a new game
```json
Request: { "player_id": "...", "game_mode": "Classic" }
Response: GameState { ... }
```

**`make_guess`** - Submit a guess
```json
Request: { "game_id": "...", "guess": "STALE" }
Response: { "feedback": [...], "is_correct": false, "result": "in_progress" }
```

**`get_hint`** - Get hint suggestions
```json
Request: { "game_id": "...", "remaining_words": [...] }
Response: HintAnalysis { 
  "top_suggestions": [GuessHint { ... }],
  "current_entropy": 4.5,
  "letter_analysis": { ... }
}
```

## Testing

### Unit Tests

Run backend tests:
```bash
cd src-tauri
cargo test
```

Run frontend tests:
```bash
npm test
```

### Test Coverage

- **engine.rs**: Exact matches, no matches, present letters, duplicate handling, case insensitivity
- **word_list.rs**: Deterministic daily words, validation, set lookup
- **hint_engine.rs**: Entropy calculation, suggestion ranking
- **db.rs**: Create/read players, game states, stats updates

## Build and Release

### Build Artifacts

**Windows**:
```bash
npm run tauri-build  # Generates .msi installer and portable .exe
```

**Linux**:
```bash
npm run tauri-build  # Generates .AppImage, .deb, and Flatpak
```

### Code Signing (CI/Secrets)

Store signing certificates in GitHub Actions secrets:
- `WINDOWS_SIGNING_CERT_BASE64`
- `WINDOWS_SIGNING_CERT_PASSWORD`

### Release Checklist

- [ ] Run full test suite (`npm test`)
- [ ] Update version in `package.json` and `Cargo.toml`
- [ ] Build artifacts for all platforms
- [ ] Create GitHub Release with checksums
- [ ] Sign binaries with certificate
- [ ] Test on fresh OS installations
- [ ] Update changelog

## Roadmap

### Phase 1 (Current)
- Core engine & evaluation
- Daily word selection
- Basic UI with React
- SQLite persistence
- Hint engine
- Basic theming

### Phase 2
- [ ] Plugin loader system (modes as plugins)
- [ ] Custom word pack support
- [ ] Achievements system
- [ ] Improved animations
- [ ] Keyboard customization

### Phase 3
- [ ] Optional cloud sync (LAN/peer-to-peer)
- [ ] Leaderboards (local + optional cloud)
- [ ] Word pack marketplace UI
- [ ] Advanced solver visualization
- [ ] Accessibility improvements

### Phase 4
- [ ] Mobile app (Flutter)
- [ ] Central server (optional)
- [ ] Custom packs submission & review
- [ ] Analytics dashboard (opt-in)

## License

MIT OR Apache-2.0 (dual licensed)

## Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create a feature branch
3. Write tests for new features
4. Submit a pull request

##  Support

-  [Tauri Docs](https://tauri.app/)
-  [React Docs](https://react.dev/)
-  [Rust Book](https://doc.rust-lang.org/book/)

---

Built with care using Tauri, React, TypeScript, and Rust
