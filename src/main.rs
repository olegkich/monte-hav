
mod board;
use board::BoardState; 

fn main() {
    let mut state = BoardState::new(3);
    state.initialize_state();

    println!("\n{:?}", state);
}

