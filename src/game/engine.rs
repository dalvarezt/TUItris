use std::time::Duration;

use super::bag::BagRandomizer;
use super::board::{Board, BUFFER_ROWS, WIDTH};
use super::rotation::try_rotate_cw;
use super::tetromino::{Piece, TetrominoType};

pub const LOCK_DELAY: Duration = Duration::from_millis(500);
const LINES_PER_LEVEL: u32 = 10;

/// Time between automatic one-cell falls for a given level 
pub fn gravity_interval(level: u32) -> Duration {
    let ms = match level {
        0 | 1 => 800,
        2 => 720,
        3 => 630,
        4 => 550,
        5 => 470,
        6 => 380,
        7 => 300,
        8 => 220,
        9 => 160,
        _ => 100, //level 10+ is capped
    };
    Duration::from_millis(ms)
}

pub fn level_for_lines(total_lines: u32) -> u32 {
    total_lines / LINES_PER_LEVEL + 1 
}

/// flat scoring: no level multiplier
pub fn score_for_lines(cleared: usize) -> u32 {
    match cleared {
        1 => 100,
        2 => 250, 
        3 => 400,
        4 => 800,
        _ => 0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineState {
    Playing, 
    GameOver,
}

/// The falling piece plus the board position of its 4x4 box's top left corner
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivePiece {
    pub piece: Piece,
    pub x: i32, 
    pub y: i32,
}

impl ActivePiece {
    /// spawn horizontally centered at row 0 (inside the hidden buffer).
    fn spawn(kind: TetrominoType) -> Self {
        let piece = Piece::new(kind);
        let x = ((WIDTH - piece.size) / 2 ) as i32;
        Self { piece, x, y: 0 }
    }
}

fn draw(bag: &mut BagRandomizer) -> TetrominoType {
    bag.next().expect("BagRandomizer never runs out")
}


pub struct Engine {
    board: Board,
    bag: BagRandomizer,
    active: ActivePiece,
    next: TetrominoType,
    score: u32,
    level: u32,
    lines: u32,
    state: EngineState,
    gravity_timer: Duration,
    lock_timer: Option<Duration>, //None = not touching the ground
}

impl Engine {
    pub fn new() -> Self {
        let mut bag = BagRandomizer::new();
        let first = draw(&mut bag);
        let next = draw(&mut bag);
        Self {
            board: Board::new(),
            bag,
            active: ActivePiece::spawn(first),
            next,
            score: 0,
            level: 1,
            lines: 0,
            state: EngineState::Playing,
            gravity_timer: Duration::ZERO,
            lock_timer: None,
        }
    }

    pub fn board(&self) -> &Board { &self.board }
    pub fn active(&self) -> &ActivePiece { &self.active }
    pub fn next_kind(&self) -> TetrominoType { self.next }
    pub fn score(&self) -> u32 { self.score }
    pub fn level(&self) -> u32 { self.level }
    pub fn lines(&self) -> u32 { self.lines }
    pub fn state(&self) -> EngineState { self.state }

    fn is_playing(&self) -> bool {
        self.state == EngineState::Playing
    }

    fn is_grounded(&self) -> bool {
        let ActivePiece{ piece, x, y } = self.active;
        !self.board.is_valid_position(&piece, x, y + 1)
    }

    /// call after anything that changes the active piece's position
    /// Grounded -> (re)start the lock time at 0. In the air -> clear it.
    fn sync_lock_state(&mut self) {
        self.lock_timer = self.is_grounded().then_some(Duration::ZERO);
    }

    fn try_shift(&mut self, dx: i32, dy: i32) -> bool {
        let ActivePiece { piece, x, y } = self.active;
        if self.board.is_valid_position(&piece, x + dx, y + dy) {
            self.active.x += dx;
            self.active.y += dy;
            self.sync_lock_state();
            true
        } else {
            false
        }
    }

    /// Left (-1) / right (+1). Returns whether the piece moved
    pub fn move_by(&mut self, dx: i32) -> bool {
        self.is_playing() && self.try_shift(dx, 0)
    }

    /// Rotate clockwise using the kick table from rotation.rs 
    pub fn rotate(&mut self) -> bool {
        if !self.is_playing() {
            return false;
        }
        let ActivePiece { piece, x, y } = self.active;
        match try_rotate_cw(&self.board, &piece, x, y) {
            Some((piece, x, y)) => {
                self.active = ActivePiece { piece, x, y };
                self.sync_lock_state();
                true
            }
            None => false, // every kick collided: rotation cancelled
        }
    }

    /// Down arrow: one cell down immediately, and restart the gravity timer.
    pub fn soft_drop(&mut self) -> bool {
        let moved = self.is_playing() && self.try_shift(0, 1);
        if moved {
            self.gravity_timer = Duration::ZERO;
        }
        moved
    }

    /// Row where the active piece would land (ghost piece / hard drop target).
    pub fn ghost_y(&self) -> i32 {
        let ActivePiece { piece, x, mut y } = self.active;
        while self.board.is_valid_position(&piece, x, y + 1){
            y += 1; 
        }
        y
    }

    /// Writes the active piece into the board, scores, then spawns the next one.
    fn lock_active(&mut self) {
        let ActivePiece { piece, x, y } = self.active;
        self.board.lock_piece(&piece, x, y);

        // Lock out: every block of the piece sits in the hidden buffer rows.
        let locked_out = piece
            .blocks()
            .all(|(_, dy)| y + dy < BUFFER_ROWS as i32);

        let cleared = self.board.clear_lines();
        self.score += score_for_lines(cleared);
        self.lines += cleared as u32;
        self.level = level_for_lines(self.lines);

        if locked_out {
            self.state = EngineState::GameOver;
        } else {
            self.spawn_next();
        }
    }
    
    /// Promotes 'next' to active and draws a new `next`.
    fn spawn_next(&mut self) {
        let kind = self.next;
        self.next = draw(&mut self.bag);
        self.active = ActivePiece::spawn(kind);
        self.gravity_timer = Duration::ZERO;
        self.sync_lock_state();

        // Block Out; the new piece overlaps existing blocks.
        let ActivePiece { piece, x, y } = self.active;
        if !self.board.is_valid_position(&piece, x, y) {
            self.state = EngineState::GameOver;
        }
    }


    /// Space: jump to the ghost row and lock immediately (bypasses lock delay).
    pub fn hard_drop(&mut self) {
        if !self.is_playing() {
            return;
        }
        self.active.y = self.ghost_y();
        self.lock_active();
    }

    pub fn tick(&mut self, dt: Duration) {
        if !self.is_playing() {
            return
        }

        // 1, Lock delay: count up while grounded.
        let lock_expired = match self.lock_timer.as_mut() {
            Some(timer) => {
                *timer += dt;
                *timer >= LOCK_DELAY
            }
            None => false,
        };
        if lock_expired {
            self.lock_active();
            return;
        }

        // 2. Gravity: possible several steps if dt was large
        self.gravity_timer += dt;
        let interval = gravity_interval(self.level);
        while self.gravity_timer >= interval {
            self.gravity_timer -= interval;
            if !self.try_shift(0, 1) {
                self.gravity_timer = Duration::ZERO; //resting; don't bank time
                break;
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}


/// TODO: write tests 
