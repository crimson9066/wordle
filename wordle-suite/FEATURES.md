# Wordle Suite - Features Guide

## 🎮 Core Gameplay

### Classic Wordle Mode
- **Rules**: Standard Wordle - 6 attempts to guess a 5-letter word
- **Feedback**: Green (correct position), Yellow (wrong position), Gray (not in word)
- **Daily Word**: One word per day, same for all players
- **Win Condition**: All letters green
- **Loss Condition**: 6 incorrect guesses

### Game Board
- Responsive tile grid (up to 6 rows × 5 columns)
- Flip animations on feedback
- Visual feedback for each letter state
- Keyboard input or on-screen keyboard

## 📊 Player Profile & Statistics

### Player Profiles
- Unique ID per install (installs_seed)
- Display name (customizable)
- Preferences/settings storage
- Created timestamp

### Game Statistics
- **Total Games**: Cumulative count
- **Total Wins**: Games won
- **Win Streak**: Current consecutive wins
- **Best Streak**: Longest streak ever
- **Avg Guesses**: Average guesses per win
- **Guess Distribution**: How many wins in 1, 2, 3, 4, 5, 6 guesses
- **Last Played**: Date of most recent game

### Achievements (Roadmap)
- First Win
- Streak Milestones (5, 10, 25, 50)
- Perfect Wins (1-guess, 2-guess)
- Speed Records
- Custom badges

## 💡 Hint & Solver System

### Entropy-Based Hints
- **Algorithm**: Information-gain calculation
- **Top 5 Suggestions**: Ranked by expected information gain
- **How It Works**:
  - Partition remaining answers by each candidate's feedback pattern
  - Calculate entropy reduction of partition
  - Suggest guesses that minimize expected remaining possibilities

### Hint Display
- Suggested word
- Entropy reduction (bits of information)
- Expected remaining answers after guess
- Plain-English explanation

### Solver Visualization (Roadmap)
- Step-through mode: Show how solver narrows down answers
- Educational: Explain why each guess is optimal
- Demonstrate partition process
- Show letter elimination progress

### Letter Analysis
- Frequency of each letter in remaining answers
- Position-by-position letter distribution
- Helps identify likely letters

## 🎨 Customization & Accessibility

### Themes
- **Light Theme**: Clean, bright UI (default)
- **Dark Theme**: Easier on eyes in low light
- **Color Blind Mode**: High-contrast palette (Deuteranopia, Protanopia support)
- **Adaptive**: Respect system dark/light preference

### Accessibility Features
- **Keyboard Navigation**: Full keyboard-first support
  - Arrow keys or WASD to navigate
  - Enter to submit guess
  - Backspace to clear
  - Full keyboard-only gameplay possible
- **Screen Reader Support**: ARIA labels on all interactive elements
- **Contrast**: WCAG AA compliant color contrast
- **Text Size**: Scalable with system font size
- **High Contrast Mode**: Supported via color-blind mode

### Settings
- Theme selection (light/dark/auto)
- Hard mode (common letters only suggestions)
- Color blind mode
- Language selection (future)
- Animation speed
- Sound effects on/off

## 🎯 Game Modes (Roadmap)

### Classic Mode (Current)
- Standard Wordle rules
- One word per day
- 6 attempts

### Unlimited Mode
- Unlimited games per day
- Random words
- Practice mode

### Timed Mode
- Solve against the clock
- 2-minute timer
- Speed ranking

### Hard Mode
- Harder word selection
- Mandatory guesses must use confirmed letters
- Restricted feedback

### Cipher Mode
- Guess words with letter substitution (ROT13)
- Higher difficulty
- Same rules otherwise

### Custom Mode (Future)
- Choose difficulty, word source, rules
- Time limits
- Handicaps

## 📦 Word Packs (Roadmap)

### Local Word Packs
- CSV format word lists
- Custom categories (animals, countries, technical terms)
- Multiple languages
- Easy sharing between devices

### Word Pack Features
- Integrity checking (SHA256 checksum)
- Author/version metadata
- Description and preview
- Rating from community

### Word Pack Marketplace (Future)
- Community-created packs
- Curated collections
- Download and manage packs
- Rate and review

## 📱 User Interface

### Main Game Screen
- Game board with tiles
- Input field or on-screen keyboard
- Current guess counter
- Result status message

### Statistics View
- Win percentage
- Current/best streak
- Average guesses
- Distribution chart
- Last played date

### Settings Panel
- Theme selection
- Hard mode toggle
- Color blind mode
- Language
- Sound/notifications
- Export/import data

### Menu & Navigation
- New game button
- View stats
- Settings
- About/Help
- Export game history

## 🔔 Notifications & Feedback

### In-Game Feedback
- Invalid word warning
- Win celebration with emoji
- Loss message with word reveal
- Streak notifications
- Milestone achievements

### System Notifications (Future)
- Daily word available
- Streak reminder
- Achievement unlocked
- New word pack available

### Sound Effects (Future)
- Tile flip sound
- Submit guess sound
- Victory fanfare
- Error sound
- Toggle-able

## 💾 Data & Persistence

### What Gets Saved
- Player profile (name, settings, installs_seed)
- Game history (all games played)
- Statistics (aggregated counts)
- Event log (actions for analytics)

### Data Export (Future)
- Export as JSON
- Export as CSV (for stats analysis)
- Include game replays
- Portable format (can import on other device)

### Data Import (Future)
- Import from JSON export
- Merge with local games
- Restore from backup

### Backup & Restore (Future)
- Automatic daily backup
- Manual backup on demand
- Encryption with password
- Version control for backups

## 🎪 Polish & Visual Features

### Animations
- **Tile Flip**: 0.6s smooth 3D flip on feedback
- **Confetti**: Particle celebration on win
- **Slide In**: Tiles slide into place
- **Shake**: Wrong guess shake animation
- **Color Pulse**: Highlight new feedback

### Visual Polish
- Smooth transitions between screens
- Loading states
- Progress indicators
- Hover effects on buttons
- Responsive layout adjustments

## 🔐 Security & Privacy Features

### Privacy Protections
- **No Telemetry**: Off by default, no data collection
- **Local Storage**: All data stays on device
- **No Accounts**: No login required, no syncing by default
- **Opt-in Sharing**: Only share data when explicitly requested

### Optional Features
- **Cloud Sync** (opt-in): Sync games across devices
- **Leaderboards** (opt-in): Optional global rankings
- **Cloud Backup** (opt-in): Store backups remotely
- **Analytics** (opt-in): Help improve the app

### Data Security
- SQLite database stored in app data directory
- Encrypted backups (with password) - future
- TLS for any network communication - future
- Integrity verification for downloaded packs - future

## 🛠️ Developer Features (Future)

### Debug Mode
- Enable via environment variable or settings
- Show current entropy
- Display remaining word count
- Reveal answer (for testing)
- Show partition details for hints

### Performance Metrics
- API response times
- Render times
- Database query times
- Hint generation speed

### Testing Tools
- Replay game feature
- Random game generator
- Bulk game simulation
- Statistics validator

## 🌐 Network Features (Roadmap)

### Local Multiplayer (Opt-in)
- LAN discovery via mDNS
- Play with friends on same network
- See friend's guesses
- Synchronized daily word

### Cloud Multiplayer (Opt-in)
- Friend lists
- Leaderboards (global, friends, daily)
- Challenges ("Can you beat my score?")
- Shared word packs

### Word Pack Distribution (Future)
- Download packs from server
- Auto-update popular packs
- Community voting

### Analytics Submission (Opt-in)
- Anonymous game statistics
- Help developers understand play patterns
- Improve word selection algorithms
- Clear privacy policy

## 📈 Metrics & Analytics

### Player Metrics
- Games per day
- Win rate by mode
- Average time per game (timed mode)
- Most difficult words
- Most used first guesses

### System Metrics
- Performance: API latency, render time
- Stability: Error rates, crash reports
- Usage: Active players, retention
- Engagement: Daily active users, session length

All analytics are:
- ✅ Opt-in (not enabled by default)
- ✅ Anonymous (no personal data)
- ✅ Encrypted (TLS in transit)
- ✅ Exportable (download your data)
- ✅ Deletable (wipe your history)

---

## Feature Roadmap

### Phase 1 (MVP - Current)
- ✅ Core Wordle gameplay
- ✅ Daily words
- ✅ Statistics tracking
- ✅ Hints & solver
- ✅ Theme support
- ✅ Accessibility basics

### Phase 2 (Polish & Modes)
- [ ] Additional game modes (Unlimited, Timed, Hard)
- [ ] Animations & visual effects
- [ ] Achievements system
- [ ] Sound effects
- [ ] Word pack system
- [ ] Enhanced accessibility

### Phase 3 (Community & Sharing)
- [ ] Game replay & sharing
- [ ] Leaderboards (local)
- [ ] Custom word packs (community)
- [ ] Social sharing (results image)
- [ ] Local multiplayer

### Phase 4 (Cloud & Mobile)
- [ ] Optional cloud sync
- [ ] Mobile app (Flutter)
- [ ] Cloud leaderboards
- [ ] Friends & challenges
- [ ] Word pack marketplace

---

Built with care for players who love word games and attention to security & privacy. Enjoy! 🎮
