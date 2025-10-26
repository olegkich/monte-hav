use std::{cmp::{max, min}, collections::HashMap, default};

#[derive(Debug)]
enum HexOwner {
    P1,
    P2,
    None
}

#[derive(Debug)]
struct Hex {
    q: i32,
    r: i32,
    owner: HexOwner 
}

#[derive(Debug)]
pub struct BoardState {
    state: HashMap<(i32, i32), Hex>,
    board_size: i8
}

impl BoardState {
    pub fn new(board_size: i8) -> Self {
        Self {state: HashMap::new(), board_size }
    }

    pub fn initialize_state(&mut self) {
        // assume the board is a pointy-bottom hex
        // the tiles are flat-bottom hexes
        let n: i8 = self.board_size * 2 - 1;

        for col in 0..n {
            for row in 0..n {
                let q: i32 = (col - (self.board_size - 1)) as i32;
                let r: i32 = (row - (self.board_size - 1)) as i32;
                self.state.insert((q, r), Hex { q, r, owner: HexOwner::None });
            } 
        }
    }

    pub fn print_state_pretty(&self) {
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


}
