
mod board;
mod win_detector;

use board::BoardState; 
use win_detector::WinDetector;

fn main() {
    let mut board = BoardState::new(4);
    board.start_game();

}

