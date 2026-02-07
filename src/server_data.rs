use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GameMoveRequest {
    pub board_state: HashMap<String, HexData>, // frontend sends as {"(q,r)": {...}}
    pub board_size: i8 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HexData {
    pub q: i32,
    pub r: i32,
    pub owner: String, // "P1" | "P2" | "None"
}

#[derive(Serialize, Deserialize)]
pub struct GameMoveResponse {
    pub best_move: (i32, i32),
    pub time_ms: u64,
}