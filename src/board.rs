use core::error;
use std::{cmp::{max, min}, collections::HashMap, default, io::stdin, string};

use crate::win_detector::{self, WinDetector};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HexOwner {
    P1,
    P2,
    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    P1,
    P2
}

impl From<&Player> for HexOwner {
    fn from(player: &Player) -> Self {
        match player {
            Player::P1 => HexOwner::P1,
            Player::P2 => HexOwner::P2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub owner: HexOwner 
}

#[derive(Debug)]
pub struct BoardState {
    pub state: HashMap<(i32, i32), Hex>,
    pub board_size: i8,
    pub turn: Player,
}

impl BoardState {
    pub fn new(board_size: i8) -> Self {
        let state = BoardState::initialize_state(board_size);

        Self {
            state,
            board_size,
            turn: Player::P1,
            }
    }

    fn initialize_state(board_size: i8) -> HashMap<(i32, i32), Hex> {
        let mut state = HashMap::new();

        // assume the board is a pointy-bottom hex
        // the tiles are flat-bottom hexes
        let n: i8 = board_size * 2 - 1;

        for col in 0..n {
            for row in 0..n {
                let q: i32 = (col - (board_size - 1)) as i32;
                let r: i32 = (row - (board_size - 1)) as i32;
                state.insert((q, r), Hex { q, r, owner: HexOwner::None });
            } 
        }

        

        return state;
    }


    // ---  API FOR MCTS --- 
    pub fn is_hex_in_bounds(&self, q: i32, r: i32) -> bool {
        let max_qr: i32 = (self.board_size - 1) as i32;

        if q.abs() > max_qr || r.abs() > max_qr { return false; }
        
        return true;
    }

    pub fn legal_moves(&self) -> Vec<(i32, i32)> {
        let mut moves: Vec<(i32, i32)> = vec![];
        
        for hex in self.state.values() {
            if hex.owner == HexOwner::None {
                moves.push((hex.q, hex.r));
            }
        };

        moves
    }

    pub fn is_terminal(&self) -> bool {
        return WinDetector::from_board(self).run(&self.turn);
    }


    pub fn apply_move(&mut self, q: i32, r: i32) -> Result<(i32, i32), &'static str> {
        if !self.is_hex_in_bounds(q, r) {
            return Err("move is out of bounds");
        }

        let hex_owner: HexOwner = HexOwner::from(&self.turn);

        self.state.insert((q, r), Hex { q, r, owner: hex_owner});

        self.next_turn();

        Ok((q, r))
    }

    // --- DEBUG AND GAME LOGIC, TO BE MOVED LATER ---

    pub fn make_move(&mut self, q: i32, r: i32) {
        let hex_owner: HexOwner = HexOwner::from(&self.turn);

        self.state.insert((q, r), Hex { q, r, owner: hex_owner});

        self.next_turn();
    }

    fn next_turn(&mut self) {
        self.turn = match self.turn {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
    }

    fn clear_screen(&self) { print!("\x1B[2J\x1B[1;1H"); }

    pub fn start_game(&mut self) {
        loop {
            // commented for debugs
            // self.clear_screen();

            print!("\n\n\n");

            self.print_state_pretty();

            // TODO: move it elsewhere
            let win_detector = WinDetector::from_board(self);

            if win_detector.run(&self.turn) {
                println!("player {:?} won", self.turn);
                return;
            }

            println!("Enter move with format: q r");

            let mut input: String = String::new();

            stdin().read_line(&mut input).unwrap();

            if input == "x" { break };

            let chars: Vec<&str> = input.split_whitespace().collect();

            let q: i32 = chars[0].trim().parse().unwrap();
            let r: i32 = chars[1].trim().parse().unwrap();

            if !self.is_hex_in_bounds(q, r) { continue; } 

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
