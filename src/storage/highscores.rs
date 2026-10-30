use serde::{Deserialize, Serialize};
use std::fs;

const MAX_ENTRIES: usize = 10;
const FILE_PATH: &str = "highscores.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighScoreEntry {
    pub initials: String, 
    pub score: u32, 
    pub level: u32,
    pub date: String,
}

pub fn load() -> Vec<HighScoreEntry> {
    let text = match fs::read_to_string(FILE_PATH) {
        Ok(t) => t,
        Err(_) => return Vec::new(), // file not yet created, start fresh
    };

    match serde_json::from_str::<Vec<HighScoreEntry>>(&text) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("Warning: high scores file format invalid. Starting with a new empty leaderboard");
            Vec::new()
        }
    }
}

pub fn save(entries: &[HighScoreEntry]) {
    match serde_json::to_string_pretty(entries) {
        Ok(json) => {
            if let Err(e) = fs::write(FILE_PATH, json) {
                eprintln!("Error: Could not write high scores file: {e}");
            }
        }
        Err(e) => eprintln!("Error serializing high scores: {e}"),
    }
}

pub fn add_entry(entries: &mut Vec<HighScoreEntry>, new_entry: HighScoreEntry) {
    entries.push(new_entry);
    entries.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(b.level.cmp(&a.level))
            .then(b.date.cmp(&a.date))
    });
    entries.truncate(MAX_ENTRIES);
}

