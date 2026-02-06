/// Board state and core game logic.
/// Contains the BoardState struct and fundamental game mechanics.

use std::collections::HashMap;

use crate::types::{Hex, HexOwner, Player};
use crate::win_detector::{self, WinDetector};

#[derive(Debug, Clone)]
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
        let max_qr: i32 = (board_size - 1) as i32;

        for q in -max_qr..=max_qr {
            for r in -max_qr..=max_qr {
                let s = -q - r;
                if s.abs() <= max_qr {
                    state.insert((q, r), Hex { q, r, owner: HexOwner::None });
                }
            }
        }

        state
    }

    // --- API FOR MCTS --- 
    
    pub fn is_hex_in_bounds(&self, q: i32, r: i32) -> bool {
        let max_qr: i32 = (self.board_size - 1) as i32;

        if q.abs() > max_qr || r.abs() > max_qr { return false; }
        
        true
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
        let detector = WinDetector::from_board(self);
        detector.run(&Player::P1) || detector.run(&Player::P2)
    }

    pub fn get_winner(&self) -> Option<Player> {
        let detector = win_detector::WinDetector::from_board(self);
        if detector.run(&Player::P1) {
            Some(Player::P1)
        } else if detector.run(&Player::P2) {
            Some(Player::P2)
        } else {
            None
        }
    }

    pub fn apply_move(&mut self, (q, r): (i32, i32)) -> Result<(i32, i32), &'static str> {
        if !self.is_hex_in_bounds(q, r) {
            return Err("move is out of bounds");
        }

        match self.state.get(&(q, r)) {
            Some(hex) => {
                if hex.owner != HexOwner::None {
                    return Err("cell already occupied");
                }
            },
            None => return Err("invalid cell"),
        }

        let hex_owner: HexOwner = HexOwner::from(&self.turn);

        self.state.insert((q, r), Hex { q, r, owner: hex_owner});

        self.next_turn();

        Ok((q, r))
    }

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        let mut board = BoardState::new(3);
        let result = board.apply_move((10, 10));
        assert!(result.is_err(), "Should return error for an out of bounds move");
    }

    #[test]
    fn test_turn_switching() {
        let mut board = BoardState::new(3);
        assert_eq!(board.turn, Player::P1);
        
        board.apply_move((0, 0)).unwrap();
        assert_eq!(board.turn, Player::P2);
        
        board.apply_move((0, 1)).unwrap();
        assert_eq!(board.turn, Player::P1);
    }

    #[test]
    fn test_prevent_overwrite() {
        let mut board = BoardState::new(3);
        board.apply_move((0, 0)).unwrap(); 
        
        let result = board.apply_move((0, 0)); 
        assert!(result.is_err(), "Should not allow moving on occupied tile");
        assert_eq!(board.turn, Player::P2, "Turn should not change on invalid move");
    }
}
