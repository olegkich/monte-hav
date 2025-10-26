
mod board;
use board::BoardState; 

fn main() {
    let mut state = BoardState::new(5);
    state.initialize_state();

    state.print_state_pretty();

}

