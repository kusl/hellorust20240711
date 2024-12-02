use crate::game_stats::GameStats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameHistory {
    pub games: Vec<GameStats>,
}
