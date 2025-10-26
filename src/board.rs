use std::{cmp::{max, min}, collections::HashMap, default, io::stdin};

#[derive(Debug)]
enum HexOwner {
    P1,
    P2,
    None
}

#[derive(Debug, PartialEq)]
enum Turn {
    P1,
    P2
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
    board_size: i8,
    turn: Turn
}

impl BoardState {
    pub fn new(board_size: i8) -> Self {
        Self {state: HashMap::new(), board_size, turn: Turn::P1 }
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

    // --- DEBUG ---

    pub fn make_move(&mut self, q: i32, r: i32) {
        let hex_owner: HexOwner;

        if self.turn == Turn::P1 { hex_owner = HexOwner::P1; }
        else {hex_owner =  HexOwner::P2;}

        self.state.insert((q, r), Hex { q, r, owner: hex_owner});
        
        self.next_turn();
    }

    fn next_turn(&mut self) {
        if self.turn == Turn::P1 { self.turn = Turn::P2; }
        else { self.turn = Turn::P2; }
    }

    fn clear_screen(&self) { print!("\x1B[2J\x1B[1;1H"); }

    pub fn start_game(&mut self) {
        loop {
            self.clear_screen();

            print!("\n\n\n");

            self.print_state_pretty();

            println!("Enter move with format: q r");

            let mut input: String = String::new();

            stdin().read_line(&mut input).unwrap();

            if input == "x" { break };

            let chars: Vec<&str> = input.split_whitespace().collect();

            let q: i32 = chars[0].trim().parse().unwrap();
            let r: i32 = chars[1].trim().parse().unwrap();

            let max_qr: i32 = (self.board_size - 1) as i32;

            if q.abs() > max_qr || r.abs() > max_qr { continue; }

            self.make_move(q, r);
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
