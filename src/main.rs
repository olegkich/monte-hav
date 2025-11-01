
mod board;
mod win_detector;
mod mcts;

use board::BoardState; 
use win_detector::WinDetector;

fn main() {
    let mut board = BoardState::new(4);
    board.start_game();
    
}

