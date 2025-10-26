
mod board;
use std::io;

use board::BoardState; 

fn main() {
    let mut state = BoardState::new(5);
    state.initialize_state();


    state.start_game();

}

