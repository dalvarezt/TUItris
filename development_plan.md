# TUItris Development Plan

**Project:** TUItris (Terminal Tetris in Rust)  
**Specification Version:** v1.0 Final (`spec.md`)  
**Target Output:** Standalone executable binary (`tuitris`)  
**Target Environment:** Linux / Unix / Cross-platform Monospace Terminal  

---

## 1. Architecture & Design Blueprint

### 1.1 Technology Stack & Crates
- **Language**: Rust (Edition 2021)
- **TUI Framework**: `ratatui` (latest 0.28+) with `crossterm` (0.28+) backend
- **Serialization**: `serde` (with `derive` feature), `serde_json`
- **Utilities**: `rand` (7-bag shuffling), `chrono` (timestamp formatting `YYYY-MM-DD`)

### 1.2 Proposed Project Structure
```
tuitris/
├── Cargo.toml
├── spec.md
├── development_plan.md
└── src/
    ├── main.rs                 # Entry point, terminal initialization, panic hook, cleanup
    ├── app.rs                  # Application state machine, event loop, view router
    ├── event.rs                # Cross-platform input & tick event handling
    ├── game/
    │   ├── mod.rs              # Game module export
    │   ├── board.rs            # 10x22 grid (20 visible + 2 buffer), line clears, cell states
    │   ├── tetromino.rs        # Piece types (I, O, T, S, Z, J, L), colors, matrix layouts
    │   ├── bag.rs              # 7-bag randomizer generator
    │   ├── rotation.rs         # Clockwise rotation and 5-step boundary/wall kicks
    │   └── engine.rs           # Core gameplay state, gravity timer, lock delay, score, levels
    ├── ui/
    │   ├── mod.rs              # Layout orchestrator, terminal size check (<60x26 warning)
    │   ├── start_menu.rs       # Title banner ASCII art & main menu navigation
    │   ├── playfield.rs        # Board rendering (10x20 visible, "██" double-width cells), ghost piece
    │   ├── sidebar.rs          # Score, level, lines, 4x4 next piece preview, keybindings
    │   ├── pause_modal.rs      # Framed "GAME PAUSED" popup dialog
    │   ├── game_over.rs        # Final stats display & 3-char high score initials input
    │   └── leaderboard.rs      # Top 10 high scores ranked table
    └── storage/
        ├── mod.rs              # Storage module export
        └── highscores.rs       # HighScore struct, file I/O (highscores.json), sorting & pruning
```

---

## 2. Phase-by-Phase Implementation Plan

### Phase 1: Project Setup & Terminal Lifecycle
**Objective:** Initialize Cargo project, configure dependencies, set up robust raw mode lifecycle and panic handling.

- [ ] **Step 1.1: Cargo Setup**
  - Initialize `Cargo.toml` with `ratatui`, `crossterm`, `serde`, `serde_json`, `rand`, and `chrono`.
  - Configure release profile (`lto = true`, `opt-level = 3`, `strip = true`) for lean standalone binary distribution.
- [ ] **Step 1.2: Terminal Raw Mode & Restoration**
  - Implement clean terminal setup (`enable_raw_mode()`, `EnterAlternateScreen`, `Hide` cursor).
  - Implement foolproof teardown (`disable_raw_mode()`, `LeaveAlternateScreen`, `Show` cursor).
  - Install a custom panic hook (`std::panic::set_hook`) to guarantee terminal restoration even upon unexpected panics.
- [ ] **Step 1.3: Event & Tick Architecture**
  - Create `event.rs` implementing an event polling loop that yields either `Event::Key(KeyEvent)` or `Event::Tick`.
  - Set a high-frequency polling interval (~16ms-33ms / 30-60 FPS) to maintain responsive keyboard controls independent of the falling gravity interval.

---

### Phase 2: Core Domain Logic & Math (Unit-Testable)
**Objective:** Build all pure game logic without UI dependencies, ensuring complete unit test coverage.

- [ ] **Step 2.1: Tetromino Matrix & Palette Definition**
  - Define enum `TetrominoType { I, O, T, S, Z, J, L }`.
  - Store standard bounding boxes for each tetromino (e.g. 4x4 for `I`, 2x2 for `O`, 3x3 for `T, S, Z, J, L`).
  - Assign palette colors according to spec:
    - `I`: Cyan
    - `O`: Yellow
    - `T`: Magenta / Purple
    - `S`: Green
    - `Z`: Red
    - `J`: Blue
    - `L`: Orange (or High-Intensity Yellow fallback)
- [ ] **Step 2.2: 7-Bag Randomizer**
  - Implement `BagRandomizer` struct containing all 7 pieces.
  - Shuffle and draw sequentially; refill with a freshly shuffled bag upon depletion.
  - Unit tests to verify fair distribution and no drought exceeding 12 pieces.
- [ ] **Step 2.3: Playfield Board Representation**
  - Create `Board` struct with width 10, total height 22 (rows 0-1 as buffer rows, rows 2-21 as visible rows).
  - Cells represent `Option<Color>` (empty or locked block).
  - Implement collision check `is_valid_position(piece, pos_x, pos_y)`.
  - Implement row clearing algorithm with line count return.
- [ ] **Step 2.4: Rotation & Boundary/Wall Kicks**
  - Implement 90° clockwise matrix rotation.
  - Implement Simplified Modern kick sequence:
    1. Standard in-place `(0, 0)`
    2. Kick left 1 cell `(-1, 0)`
    3. Kick right 1 cell `(+1, 0)`
    4. Kick left/right 2 cells for `I` piece `(±2, 0)`
    5. Kick up 1 cell `(0, -1)`
  - Revert rotation if all test offsets collide.
  - Unit test boundary kicks against left wall, right wall, and floor.

---

### Phase 3: Game Engine & Timers
**Objective:** Implement gravity scaling, soft/hard drop, lock delay, and score calculation.

- [ ] **Step 3.1: Gravity & Level Progression**
  - Starting level: 1. Advance +1 level every 10 cleared lines.
  - Map level to gravity interval:
    - Level 1: 800 ms
    - Level 2: 720 ms
    - Level 3: 630 ms
    - Level 4: 550 ms
    - Level 5: 470 ms
    - Level 6: 380 ms
    - Level 7: 300 ms
    - Level 8: 220 ms
    - Level 9: 150 ms
    - Level 10+: 100 ms (capped)
- [ ] **Step 3.2: Lock Delay Mechanism**
  - When active piece lands on stack/floor via gravity or soft drop, activate a ~500ms lock delay timer.
  - If player moves or rotates successfully while timer is active, reset/extend lock delay.
  - Lock immediately if 500ms elapses without further downward movement.
- [ ] **Step 3.3: Dropping Actions & Ghost Piece Calculation**
  - **Soft Drop (`↓`)**: Move 1 cell down immediately, reset gravity step timer.
  - **Hard Drop (`Space`)**: Raycast downward to the lowest valid position, lock immediately, bypass lock delay.
  - **Ghost Piece**: Calculate landing row `ghost_y` for visual projection.
- [ ] **Step 3.4: Flat Scoring System**
  - 1 row (Single): 100 points
  - 2 rows (Double): 250 points
  - 3 rows (Triple): 400 points
  - 4 rows (Tetris): 800 points
- [ ] **Step 3.5: Spawn & Game Over Triggers**
  - Spawn centered at row 0 horizontally.
  - Check **Block Out** (immediate collision on spawn).
  - Check **Lock Out** (blocks locked entirely above row 2 in the hidden buffer).
  - Transition engine state to `GameOver` when triggered.

---

### Phase 4: State Machine & UI Views
**Objective:** Construct Ratatui visual components, navigation states, and responsive layout.

- [ ] **Step 4.1: Adaptive Terminal Resize Warning**
  - Detect terminal dimensions on draw.
  - If width < 60 or height < 26, render a centered warning dialog: `Terminal size too small. Please resize to at least 60x26`.
- [ ] **Step 4.2: Main Menu Screen (`State::StartMenu`)**
  - Centered ASCII art banner `TUItris`.
  - Menu options: `[Enter] Start New Game`, `[H] View High Scores`, `[Q] Quit`.
- [ ] **Step 4.3: Playing Screen Layout (`State::Playing`)**
  - Centered split layout:
    - **Left Panel (Board)**:
      - Bordered block.
      - 10 columns × 20 visible rows.
      - Each cell rendered as double-width characters: `"██"` for solid blocks, `" ."` or `"  "` for empty cells.
      - Ghost piece rendered with dimmed/faint color or hollow styling (`"::"` / `"[]"`).
    - **Right Panel (Sidebar)**:
      - Score, Level, Lines Cleared cards.
      - Next Piece Box: Framed 4×4 preview with incoming tetromino shape centered.
      - Controls / Help box.
- [ ] **Step 4.4: Paused Dialog Overlay (`State::Paused`)**
  - Render semi-transparent backdrop or centered pop-up dialog with `GAME PAUSED`.
  - Keys: `P` to resume, `Q` to forfeit/exit to main menu.
- [ ] **Step 4.5: Game Over & Initials Entry Screen (`State::GameOver`)**
  - Display final score, level, lines cleared.
  - High score check:
    - If qualifying for top 10: interactive text input prompting for **3 uppercase alphanumeric initials**.
    - Support typing (`A-Z`, `0-9`), `Backspace`, `Enter` to confirm, `Esc` or `Q` to discard.
    - If not qualifying: prompt `[Enter]` to view leaderboard or `[R]` to restart.
- [ ] **Step 4.6: Leaderboard Screen (`State::Leaderboard`)**
  - Formatted table displaying Top 10 scores: Rank (1st-10th), Initials (3 chars), Score, Level, Date (`YYYY-MM-DD`).
  - Keybindings: `[Enter]` / `[R]` for new game, `[Esc]` / `[Q]` to return to main menu.

---

### Phase 5: High Score Persistence (`highscores.json`)
**Objective:** Implement robust persistence, sorting, ranking, and error-tolerant file I/O.

- [ ] **Step 5.1: HighScore Data Model**
  - Define `HighScoreEntry`:
    ```rust
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct HighScoreEntry {
        pub initials: String,
        pub score: u32,
        pub level: u32,
        pub date: String,
    }
    ```
- [ ] **Step 5.2: Storage Logic & Sorting**
  - Target file path: `./highscores.json` in the current working directory.
  - Sorting criteria: Descending by `score`, secondary by `level`, tertiary by `date`.
  - Maximum 10 entries retained.
- [ ] **Step 5.3: Error Resilience**
  - If `highscores.json` is missing: initialize empty list (do not crash).
  - If `highscores.json` is corrupted or invalid JSON: log warning, fallback to empty leaderboard without crashing.
  - Auto-create and format `highscores.json` on the first qualifying score save.

---

### Phase 6: Verification, Polishing & Quality Assurance
**Objective:** End-to-end integration testing, controls verification, performance optimization, and release build.

- [ ] **Step 6.1: Game Mechanics Verification**
  - Verify 7-bag fairness across long play sessions.
  - Test wall kick scenarios against borders and existing blocks.
  - Verify lock delay reset on slide/rotate near ground.
  - Verify gravity scaling at levels 1 through 10.
- [ ] **Step 6.2: Terminal & Input Verification**
  - Confirm double-width cell alignment on standard terminal fonts.
  - Test resizing terminal window during game, pause, and menu states.
  - Ensure immediate response to soft drop, hard drop, and rotation.
- [ ] **Step 6.3: Static Analysis & Compilation**
  - Run `cargo fmt --check`.
  - Run `cargo clippy -- -D warnings`.
  - Run `cargo test` on all unit tests.
  - Build standalone release binary: `cargo build --release`.

---

## 3. Risk Assessment & Mitigation

| Potential Risk | Impact | Mitigation Strategy |
|:---|:---|:---|
| **Terminal stays in raw mode upon crash/panic** | High | Implement `std::panic::set_hook` early in `main()` to restore cursor and alternate screen before printing panic trace. |
| **Monospace font cell aspect ratio distortion** | Medium | Use 2-character wide cells (`"██"`) for board columns to achieve an approximate 1:1 square ratio in terminal fonts. |
| **Input lag or timing drift during fast gravity** | Medium | Decouple game tick loop from keyboard event poll. Use non-blocking event polling with a 16ms timeout so inputs process instantaneously regardless of gravity interval. |
| **Corrupted `highscores.json` crashing the binary** | Low | Wrap file read and deserialization in `Result` handlers with graceful fallback to an empty vector if parsing fails. |

---

## 4. Acceptance Criteria & Definition of Done

1. **Compilation**: `cargo build --release` produces a single standalone binary `tuitris` without warnings.
2. **Gameplay Compliance**: All mechanics from `spec.md` (7-bag randomizer, 5-step kicks, lock delay, flat scoring, gravity progression) function accurately.
3. **UI Fidelity**: Visual playfield centered with double-width cells, ghost piece, 4x4 preview box, and terminal resize warning below 60x26.
4. **State Machine**: Seamless flow across Start Menu, Playing, Paused, Game Over, and Leaderboard.
5. **Persistence**: Scores accurately recorded and persisted to `./highscores.json` with top-10 ranking and resilient error handling.
6. **Code Quality**: Passes `cargo test`, `cargo fmt`, and `cargo clippy`.

