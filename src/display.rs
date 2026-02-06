/// Display/rendering logic for the Havannah board.
/// Handles terminal output and visual representation.

use crate::board::BoardState;
use crate::types::HexOwner;

pub trait BoardDisplay {
    fn print_state_pretty(&self);
    fn print_state_less_pretty(&self);
    fn clear_screen(&self);
}

impl BoardDisplay for BoardState {
    fn print_state_pretty(&self) {
        let n = (self.board_size - 1) as i32; 
        
        for r in -n..=n {
            let q_min = (-n).max(-r - n);
            let q_max = n.min(-r + n);
            let row_length = (q_max - q_min + 1) as usize;
            
            let max_length = (2 * self.board_size - 1) as usize;
            let indent_count = max_length - row_length;
            print!("{}", " ".repeat(indent_count));
            
            for q in q_min..=q_max {
                let key = (q as i32, r as i32);
                let symbol = match self.state.get(&key) {
                    Some(hex) => match hex.owner {
                        HexOwner::None => '.',
                        HexOwner::P1 => 'X',
                        HexOwner::P2 => 'O',
                    },
                    None => '.',
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }

    fn print_state_less_pretty(&self) {
        for (q, r) in self.state.keys() {
            println!("{} {}", q, r);
        }
    }

    fn clear_screen(&self) { 
        print!("\x1B[2J\x1B[1;1H"); 
    }
}
