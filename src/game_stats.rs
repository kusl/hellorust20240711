// src/game_stats.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameStats {
    pub attempts: Vec<u32>,
    pub secret_number: u32,
    pub guesses: Vec<u32>,
}
