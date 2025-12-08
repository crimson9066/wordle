# Getting Started with Wordle Suite

## 📋 Prerequisites

Before you start, ensure you have:

- **Rust 1.70+** - [Install from https://rustup.rs/](https://rustup.rs/)
  ```bash
  # Verify installation
  rustc --version
  cargo --version
  ```

- **Node.js 18+** - [Install from https://nodejs.org/](https://nodejs.org/)
  ```bash
  # Verify installation
  node --version
  npm --version
  ```

- **Git** - For version control (optional but recommended)

## 🚀 Quick Start (5 minutes)

### 1. Navigate to Project Directory
```bash
cd d:\Projects\wordle\wordle-suite
```

### 2. Install Dependencies
```bash
npm install
```
This installs both Node packages and prepares Rust dependencies.

### 3. Start Development Server
```bash
npm run tauri-dev
```

This will:
- Start Vite dev server (React with hot reload)
- Compile and run Rust backend
- Open the app in a new window

🎉 **You're now running Wordle Suite!**

## 📁 Project Structure at a Glance

```
wordle-suite/
├── src/                    React frontend (TypeScript)
│   ├── App.tsx            Main game component
│   ├── main.tsx           React entry point
│   ├── types/             TypeScript type definitions
│   ├── store/             Zustand state management
│   └── api/               Tauri API client
│
├── src-tauri/             Rust backend (Tauri)
│   └── src/
│       ├── main.rs        App entry point & IPC handlers
│       ├── engine.rs      Game logic (core algorithm)
│       ├── word_list.rs   Daily words & validation
│       ├── hint_engine.rs Solver with entropy calculation
│       ├── db.rs          SQLite persistence
│       └── commands.rs    Tauri IPC command handlers
│
├── package.json           NPM dependencies
├── Cargo.toml             Rust workspace config
├── README.md              Full documentation
├── ARCHITECTURE.md        Technical design
└── FEATURES.md           Feature specification
```

## 🎮 How to Play

1. **Start a game**: Click "New Game" or it starts automatically
2. **Type a guess**: Enter any 5-letter word
3. **Submit**: Press Enter or click "Submit"
4. **Interpret feedback**:
   - 🟩 **Green**: Correct letter in correct position
   - 🟨 **Yellow**: Correct letter in wrong position
   - ⬜ **Gray**: Letter not in the word
5. **Win**: Get all letters green in 6 attempts or fewer
6. **Hints**: Click "Hints" to get suggestions (Entropy-based solver)

## 🛠️ Development Tasks

### Run Development Mode
```bash
npm run tauri-dev
```
- Hot reload for React changes
- Rebuilds Rust on file save
- Opens app in debug window

### Build for Production
```bash
npm run tauri-build
```
Produces:
- Windows: `.msi` installer + portable `.exe`
- Linux: `.AppImage` + `.deb` + Flatpak
- macOS: `.dmg` (if building on macOS)

### Run Tests
```bash
npm test              # React tests
cd src-tauri && cargo test && cd ..  # Rust tests
```

### Run Linters
```bash
npm run lint          # ESLint for TypeScript
cd src-tauri && cargo clippy && cd ..  # Clippy for Rust
```

### Type Checking
```bash
npm run type-check    # TypeScript compiler check
```

## 📚 Key Files to Explore

### Backend (Learn the Game Logic)
1. **`src-tauri/src/engine.rs`** (100 lines)
   - Core algorithm: `evaluate_guess()`
   - Two-pass evaluation handles duplicate letters
   - Heavily commented and tested

2. **`src-tauri/src/word_list.rs`** (200 lines)
   - Daily word selection algorithm
   - Deterministic scheduling
   - Word validation

3. **`src-tauri/src/hint_engine.rs`** (300 lines)
   - Information-gain solver
   - Entropy calculations
   - Suggestion ranking

### Frontend (Learn React + Tauri)
1. **`src/App.tsx`** (150 lines)
   - Main game board component
   - Input handling
   - Game flow

2. **`src/api/gameApi.ts`** (100 lines)
   - Tauri IPC wrapper
   - Type-safe command calls

3. **`src/store/appStore.ts`** (150 lines)
   - Zustand state management
   - Player, game, and stats state

## 🐛 Common Issues & Troubleshooting

### "cargo: command not found"
- **Fix**: Install Rust from https://rustup.rs/
- **Windows**: May need to restart terminal after install

### "npm: command not found"
- **Fix**: Install Node.js from https://nodejs.org/
- **Windows**: May need to restart terminal after install

### "Database locked" error
- **Fix**: Delete `%APPDATA%\wordle-suite\wordle.db` and restart
- **Cause**: Multiple instances or previous crash

### Tauri window won't open
- **Fix**: Check app data directory exists
- **Windows**: `%APPDATA%\Roaming\wordle-suite`
- **Linux**: `~/.local/share/wordle-suite`

### TypeScript errors in IDE
- **Fix 1**: Run `npm install` to ensure all packages installed
- **Fix 2**: Restart TypeScript server (Cmd+Shift+P → "Restart TS Server")
- **Fix 3**: Delete `node_modules` and `npm install` again

### Hot reload not working
- **Fix 1**: Stop dev server (Ctrl+C) and run `npm run tauri-dev` again
- **Fix 2**: Check if port 5173 is in use
- **Fix 3**: In VS Code, close and reopen the terminal

## 📖 Documentation Map

- **[README.md](./README.md)** - Project overview and features
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical design and data flow
- **[FEATURES.md](./FEATURES.md)** - Feature specification and roadmap
- **[.github/copilot-instructions.md](./.github/copilot-instructions.md)** - Development guide
- **Code comments** - Inline documentation in source files

## 🎯 Next Steps After Setup

### If you want to...

#### **Understand the code**
1. Read `ARCHITECTURE.md` for system design
2. Start with `src-tauri/src/engine.rs` (core algorithm)
3. Explore `src/App.tsx` (UI component)
4. Check the test files for usage examples

#### **Add a feature**
1. Plan in [FEATURES.md](./FEATURES.md) roadmap
2. Add logic in Rust (backend)
3. Add API call in `gameApi.ts` (IPC)
4. Add UI in React component (frontend)
5. Add tests and types

#### **Deploy to users**
1. Build: `npm run tauri-build`
2. Find artifacts in `src-tauri/target/release/bundle/`
3. Create GitHub Release with checksums
4. Users can download and install

#### **Extend with plugins** (Future)
1. Create plugin module in Rust
2. Implement `Plugin` trait
3. Expose via Tauri command
4. Load dynamically at startup

## 💡 Tips & Tricks

### Dev Experience
- **Hot reload**: Changes to `.tsx` files reload instantly
- **Quick compile**: Rust incremental compilation is fast
- **Dev tools**: Open browser DevTools with F12
- **Logs**: Check browser console and terminal for errors

### Productivity
- **Use VS Code Extensions**:
  - Rust-analyzer (for Rust)
  - ES7+ React/Redux/React-Native snippets
  - Tauri snippets
- **Terminal**: Use two terminals:
  - One for `npm run tauri-dev`
  - One for git/build commands

### Performance
- **Lazy load modules**: React.lazy() for code splitting
- **Memoize components**: React.memo() for expensive renders
- **Cache results**: HintEngine caches suggestions per game

## 🤝 Contributing

Want to improve Wordle Suite? Great! Here's how:

1. Create a branch: `git checkout -b feature/your-feature`
2. Make changes following the code style
3. Add tests: `cargo test` and `npm test`
4. Commit: `git commit -m "feat: description"`
5. Push and create a pull request

See [README.md](./README.md#contributing) for full guidelines.

## 📞 Support

- 📖 **Tauri docs**: https://tauri.app/
- 📖 **React docs**: https://react.dev/
- 📖 **Rust book**: https://doc.rust-lang.org/book/
- 💬 **GitHub Issues**: Report bugs or ask questions

## 🎓 Learning Resources

### Understand the Core Algorithm
- Start: `src-tauri/src/engine.rs` - The core `evaluate_guess()` function
- Test: Run `cargo test engine::` to see it in action
- Try: Modify the test cases to experiment

### Understand the Solver
- Start: `src-tauri/src/hint_engine.rs` - Information-gain calculation
- Concept: Shannon entropy in information theory
- Visual: Try the hints feature in-game

### Understand the IPC
- Start: `src-tauri/src/commands.rs` - Handler definitions
- Next: `src/api/gameApi.ts` - React wrapper
- Try: Add a new command (e.g., `get_game_history`)

## ✅ Verification Checklist

Before committing or deploying, ensure:

- [ ] `npm run type-check` passes (no TypeScript errors)
- [ ] `npm run lint` passes (no linting errors)
- [ ] `npm test` passes (tests pass)
- [ ] `cd src-tauri && cargo test && cd ..` passes (Rust tests pass)
- [ ] `npm run tauri-dev` launches without errors
- [ ] Can start a game, make guesses, win/lose
- [ ] Stats update correctly
- [ ] Theme toggle works
- [ ] Hints generate suggestions

---

**Ready to build something amazing?** Start with `npm run tauri-dev` and have fun! 🚀

Questions? Check the docs or open an issue on GitHub!
