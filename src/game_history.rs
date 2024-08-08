use serde::{Deserialize, Serialize};
use crate::game_stats::GameStats;

#[derive(Serialize, Deserialize)]
pub struct GameHistory {
    pub games: Vec<GameStats>,
}
