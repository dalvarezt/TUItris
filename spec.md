# TUItris - Terminal Tetris Specification (v1.0 Final)

A full-featured, terminal-based Tetris game written in Rust that compiles into a single standalone executable.

---

## 1. System & Architecture Overview
- **Language & Runtime**: Rust (Edition 2021).
- **Binary Output**: Single standalone executable binary (`tuitris`).
- **Terminal UI Library**: `ratatui` with `crossterm` backend for raw mode terminal handling, double-buffering, and cross-platform event dispatch.
- **Dependencies**: Minimal dependencies: `crossterm`, `ratatui`, `serde`, `serde_json`, `rand`, `chrono` (or simple timestamp formatting).

---

## 2. Layout & Visual Display
The game interface is rendered using Ratatui layout blocks centered in the terminal window (minimum required terminal size: ~60 columns × 26 rows, with an adaptive warning if resized below minimum).

### 2.1 Screen Layout
- **Left Panel (Playfield / Board)**:
  - Width: 10 columns.
  - Height: 20 visible rows (with 2 hidden buffer rows at the top for piece spawning and rotation handling).
  - Cell Aspect Ratio: Each cell is rendered using two characters (`"██"` for solid filled blocks, or styled empty cells `" ."` / `"  "`) to achieve an approximately 1:1 square visual appearance in standard terminal monospace fonts.
  - Border: Clean boxed border (e.g. `ratatui::widgets::Block::bordered()`).
- **Right Panel (Information & Controls)**:
  - **Score**: Current accumulated points.
  - **Level**: Current difficulty level (starts at 1).
  - **Lines Cleared**: Total count of completed rows in the current session.
  - **Next Piece Box**: A framed 4×4 preview showing the next incoming tetromino shape centered.
  - **Help / Keybindings Box**: Quick reference of active control keys.

### 2.2 Color Palette
Colors are rendered using ANSI/256/RGB terminal color capabilities (with graceful fallback if truecolor is unavailable):
- **I-Piece**: Cyan
- **O-Piece**: Yellow
- **T-Piece**: Magenta / Purple
- **S-Piece**: Green
- **Z-Piece**: Red
- **J-Piece**: Blue
- **L-Piece**: Orange (or High-Intensity Yellow/White fallback)
- **Ghost Piece**: Subtle, dimmed/faint color or hollow bracket styling (`"::"` / `"[]"`) indicating where the active piece will land upon hard drop.
- **Locked/Stack Blocks**: Displayed in their respective piece colors.

---

## 3. Game Mechanics

### 3.1 Tetromino Generation (7-Bag Randomizer)
- Uses the standard **7-bag randomizer** algorithm:
  - A "bag" containing all 7 tetrominoes (`I`, `O`, `T`, `S`, `Z`, `J`, `L`) is shuffled at random.
  - Pieces are drawn sequentially from the bag.
  - When the bag is depleted, a new shuffled 7-piece bag is generated.
  - Guarantees no piece drought (maximum 12 pieces between any two occurrences of the same tetromino).

### 3.2 Rotation & Boundary Kicks
- **Rotation**: Standard 90-degree clockwise matrix rotation about the piece center.
- **Boundary & Collision Kicks (Simplified Modern)**:
  - When a piece rotates near the left wall, right wall, floor, or adjacent stack blocks, the game checks offsets:
    1. Standard in-place rotation (dx: 0, dy: 0).
    2. Kick left 1 cell (dx: -1, dy: 0) if colliding with the right wall/block.
    3. Kick right 1 cell (dx: +1, dy: 0) if colliding with the left wall/block.
    4. Kick left/right 2 cells for long `I` piece when necessary.
    5. Kick up 1 cell (dx: 0, dy: -1) if colliding with floor/stack.
  - If all test offsets fail collision checks, the rotation is cancelled and the piece remains in its original orientation.

### 3.3 Movement & Dropping
- **Horizontal Move**: Left Arrow (←) and Right Arrow (→) move the piece 1 cell horizontally if unoccupied.
- **Soft Drop**: Down Arrow (↓) moves the piece downward 1 cell immediately and resets the step gravity timer.
- **Hard Drop**: Spacebar instantly drops the piece down to the ghost position and immediately locks it into the stack.
- **Lock Delay**: When a piece contacts the floor or locked stack via gravity or soft drop, a ~500ms lock delay allows the player to perform final slide/rotation adjustments before the piece locks into the grid.

### 3.4 Spawn & Game Over
- Active piece spawns centered at row 0 (or buffer rows) horizontally.
- **Game Over Condition**: If a newly spawned piece collides with existing locked blocks immediately upon spawn (Block Out), or if blocks lock above the 20 visible rows (Lock Out), the game ends.

---

## 4. Scoring & Difficulty Progression

### 4.1 Flat Scoring System
Points are strictly awarded based on the number of rows simultaneously cleared in a single drop (no level multiplier, flat score):
- **1 row (Single)**: 100 points
- **2 rows (Double)**: 250 points
- **3 rows (Triple)**: 400 points
- **4 rows (Tetris)**: 800 points

### 4.2 Level & Speed Scaling
- Starting level: **Level 1**.
- Level advancement: Every **10 cleared rows** increments the level by 1 (e.g. Level 2 at 10 rows, Level 3 at 20 rows, etc.).
- Fall speed (Gravity interval) scales down smoothly per level:
  - Level 1: 800 ms per step
  - Level 2: 720 ms per step
  - Level 3: 630 ms per step
  - Level 4: 550 ms per step
  - Level 5: 470 ms per step
  - Level 6: 380 ms per step
  - Level 7: 300 ms per step
  - Level 8: 220 ms per step
  - Level 9: 150 ms per step
  - Level 10+: 100 ms per step (caps at maximum speed)

---

## 5. Game States & Navigation Flow

The game operates as a finite state machine:
```
           ┌──────────────┐
           │  Start Menu  │◄─────────────┐
           └──────┬───────┘              │
                  │ 'Enter'              │
                  ▼                      │
           ┌──────────────┐       ┌──────┴───────┐
    ┌─────►│   Playing    │◄─────►│    Paused    │
    │      └──────┬───────┘ 'P'   └──────────────┘
    │             │ Game Over
    │             ▼
    │      ┌──────────────┐
    │      │  Game Over / │
    │      │Initials Entry│
    │      └──────┬───────┘
    │             │ 'Enter' or Discard
    │             ▼
    │      ┌──────────────┐
    │      │ Leaderboard  ├──────────────┘
    └──────┤ (Highscores) │ 'Enter' / 'R' / 'Esc'
           └──────────────┘
```

1. **Start Screen / Main Menu**:
   - Title ASCII art banner (`TUItris`).
   - Options:
     - `[Enter]` Start New Game
     - `[H]` View High Scores
     - `[Q]` Quit
2. **Playing State**:
   - Active gameplay with real-time ticker and keyboard event handling.
   - Controls:
     - `←` / `→`: Move piece left / right.
     - `↑`: Rotate piece 90° clockwise.
     - `↓`: Soft drop (move down 1 cell).
     - `Space`: Hard drop (instant drop & lock).
     - `P`: Pause game.
     - `Q`: Forfeit game and return to main menu.
3. **Paused State**:
   - Renders a semi-transparent or bordered pop-up dialog with `GAME PAUSED`.
   - `P` resumes gameplay.
   - `Q` exits to main menu.
4. **Game Over / Initials Entry**:
   - Displays final score, level, and lines cleared.
   - **High Score Qualification**:
     - Evaluated against `highscores.json`. If score ranks in the top 10 (or if fewer than 10 scores exist):
       - An interactive text input box asks for **3 Initials** (e.g. `ABC`).
       - Characters entered are automatically uppercase alphanumeric.
       - Backspace allows correcting input.
       - Pressing `Enter` with 3 initials submits and saves the score to `highscores.json`.
       - If the player leaves initials blank, cancels, or presses `Esc`/`Q` to exit without providing 3 initials, the score is discarded.
     - If the score does not qualify for top 10:
       - Displays "Game Over" message with prompt to press `Enter` to view leaderboard or `R` to restart.
5. **Leaderboard / High Scores Screen**:
   - Renders a ranked table (Rank #1 to #10) showing:
     - Rank (1st - 10th)
     - Initials (3 chars)
     - Score
     - Level reached
     - Date (YYYY-MM-DD)
   - `[Enter]` / `[R]` to start new game, `[Esc]` / `[Q]` to return to main menu.

---

## 6. High Score Persistence (`highscores.json`)

### 6.1 Storage Path
- File path: `./highscores.json` (stored in the current working directory alongside the executable).
- The file is **not required** at initial launch; it is automatically created when the first qualifying high score is saved.
- If the file is missing, empty, or corrupted, the game handles it gracefully without crashing (falling back to an empty leaderboard).

### 6.2 JSON Schema
```json
[
  {
    "initials": "ALX",
    "score": 3200,
    "level": 4,
    "date": "2026-10-02"
  }
]
```
- Maximum 10 entries stored, sorted descending by `score` (and secondary sort by `level`, then `date`).

---

## 7. Controls Summary Table

| Key | Context | Action |
|:---|:---|:---|
| `←` / `→` | Playing | Move piece left / right |
| `↑` | Playing | Rotate piece 90° clockwise (with boundary kicks) |
| `↓` | Playing | Soft drop 1 cell down |
| `Space` | Playing | Hard drop (instant drop & lock) |
| `P` | Playing / Paused | Toggle Pause / Resume |
| `Q` | Playing / Menus | Forfeit / Quit / Return to Menu |
| `A-Z`, `0-9` | High Score Input | Type 3 initials |
| `Backspace` | High Score Input | Delete previous character |
| `Enter` | High Score Input | Confirm 3 initials & save score |
| `Esc` | High Score Input | Discard score without saving |
| `Enter` / `R` | Menus / Game Over | Start / Restart Game |
| `H` | Main Menu | View Leaderboard |
