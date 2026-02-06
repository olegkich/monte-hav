
mod types;
mod board;
mod display;
mod game;
mod win_detector;
mod mcts;

use board::BoardState;
use game::GameRunner;

fn main() {
    let mut board = BoardState::new(5);
    board.start_game_ai_vs_ai(500, Some(6));
}

