use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent};

use crate::game::engine::{Engine, EngineState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    StartMenu,
    Playing,
    Paused,
    GameOver,
    Leaderboard,
}

#[derive(Clone, Debug)]
pub struct HighScoreEntry {
    pub initials: String,
    pub score: u32,
    pub level: u32,
    pub date: String,
}
    
pub struct App {
    state: AppState,
    engine: Engine,
    initials_input: String,
    high_scores: Vec<HighScoreEntry>,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::StartMenu,
            engine: Engine::new(),
            initials_input: String::new(),
            high_scores: Vec::new(),
            should_quit: false,
        }
    }

    pub fn state(&self) -> AppState { self.state }
    pub fn engine(&self) -> &Engine { &self.engine }
    pub fn initials_input(&self) -> &str {&self.initials_input }
    pub fn high_scores(&self) -> &[HighScoreEntry] { &self.high_scores }
    pub fn should_quit(&self) -> bool { self.should_quit }

    pub fn is_high_score(&self) -> bool {
        let score = self.engine.score();
        if score == 0 {
            return false;
        }
        self.high_scores.len() < 10 || self.high_scores.iter().any(|s| score > s.score )
    }

    pub fn start_game(&mut self) {
        self.engine = Engine::new();
        self.initials_input.clear();
        self.state = AppState::Playing;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::StartMenu => match key.code {
                KeyCode::Enter => self.start_game(),
                KeyCode::Char('h') | KeyCode::Char('H') => self.state = AppState::Leaderboard,
                KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true, 
                _ => {}
            },
            AppState::Playing => match key.code {
                KeyCode::Left => { self.engine.move_by(-1); }
                KeyCode::Right => { self.engine.move_by(1); }
                KeyCode::Up => { self.engine.rotate(); }
                KeyCode::Down => { self.engine.soft_drop(); }
                KeyCode::Char(' ') => { self.engine.hard_drop(); }
                KeyCode::Char('p') | KeyCode::Char('P') => { self.state = AppState::Paused; }
                KeyCode::Char('q') | KeyCode::Char('Q') => { self.state = AppState::StartMenu; }
                _ => {}
            },
            AppState::Paused => match key.code {
                KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Esc => self.state = AppState::Playing,
                KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::StartMenu,
                _ => {}
            }, 
            AppState::GameOver => {
                if self.is_high_score() {
                    match key.code {
                    KeyCode::Char(c) if c.is_ascii_alphanumeric() && self.initials_input.len() < 3 => {
                        self.initials_input.push(c.to_ascii_uppercase())
                    }
                    KeyCode::Backspace => {
                        self.initials_input.pop();
                    }
                    KeyCode::Enter if self.initials_input.len() == 3 => {
                        self.save_high_score();
                        self.state = AppState::Leaderboard;
                    }
                    KeyCode::Esc => {
                        self.state = AppState::Leaderboard;
                    }
                    _ => {}
                }
                } else {
                    match key.code {
                        KeyCode::Enter => self.state = AppState::Leaderboard,
                        KeyCode::Char('r') | KeyCode::Char('R') => self.start_game(),
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.state = AppState::StartMenu,
                        _ => {}
                    }
                }
            }
            AppState::Leaderboard => match key.code {
                KeyCode::Enter | KeyCode::Char('r') | KeyCode::Char('R') => self.start_game(),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::StartMenu,
                _ => {}
            },
        }
    }

    fn save_high_score(&mut self) {
        let entry = HighScoreEntry {
            initials: self.initials_input.clone(),
            score: self.engine.score(),
            level: self.engine.level(),
            date: "1970-01-01".to_string(), //TODO: get current date
        };
        self.high_scores.push(entry);
        self.high_scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.high_scores.truncate(10);
        // TODO: store scores on disk
    }

    pub fn tick(&mut self, dt: Duration) {
        if self.state == AppState::Playing {
            self.engine.tick(dt);
            if self.engine.state() == EngineState::GameOver {
                self.state = AppState::GameOver;
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
