use std::{collections::{HashSet, VecDeque}, sync::Arc};

use crate::board::{self, BoardState, Hex, HexOwner, Player};

pub struct WinDetector<'a>{
    board: &'a BoardState,
    corners: Vec<(i32, i32)>,
    edges: Vec<(i32, i32)>
}

impl<'a> WinDetector<'a> {
    pub fn from_board(board: &'a BoardState) -> Self {
        let corners = Self::initialize_corners(board.board_size);
        let edges = Self::initialize_edges();

        Self { board, corners, edges }
    }

    pub fn run(&self, player: &Player) -> bool {
        return self.check_bridge(player)
    }

    // WANRING: can panic, there's no checking if a hex is out of bounds.
    fn get_neighbours(&self, q: &i32, r: &i32) -> Vec<(i32, i32)> {
        let q_val = *q;
        let r_val = *r;
        
        let neighbours = vec![
            (q_val + 1, r_val),
            (q_val - 1, r_val),
            (q_val, r_val + 1),
            (q_val, r_val - 1),
            (q_val + 1, r_val - 1),
            (q_val - 1, r_val + 1)
        ];

        neighbours
    }

    fn initialize_corners(board_size: i8) -> Vec<(i32, i32)> {

        let max_qr: i32 = (board_size - 1).into();

        return vec![
            (-max_qr, 0),
            (-max_qr, max_qr),
            (0, -max_qr),
            (0, max_qr),
            (max_qr, -max_qr),
            (max_qr, 0)
        ];        
    }

    fn initialize_edges() -> Vec<(i32, i32)> {
        return vec![];
    }

    fn check_ring() {

    }

    fn check_bridge(&self, player: &Player) -> bool {
        let seen: HashSet<Hex> = HashSet::new();

        for ((q, r), hex) in &self.board.state {
            if seen.contains(hex) { continue; };

            if hex.owner != HexOwner::from(player) { continue; };

            let corners = self.findConnection(&hex.q, &hex.r, &player);
            
            println!("{:?}", corners);

            if corners.len() >= 2 { return true };
        };

        return false;
    }
    
    fn findConnection(&self, start_q: &i32, start_r: &i32, player: &Player) -> HashSet<(i32, i32)> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut corners_found: HashSet<(i32, i32)> = HashSet::new();

        let mut queue: VecDeque<(i32, i32)> = VecDeque::from([(*start_q, *start_r)]);

        while queue.len() > 0 {
            let (q, r) = queue.pop_front().unwrap();

            if (self.is_corner(&q, &r)) {
                corners_found.insert((q, r));
            }

            let neighbours = self.get_neighbours(&q, &r);

            for (qn, rn) in neighbours {
                if visited.contains(&(qn, rn)) { continue; }
                
                
                if (self.get_hex_owner(&q, &r) == HexOwner::from(player)) {
                    visited.insert((qn, rn));
                    queue.push_front((qn, rn));
                }                
            }
        }
        return corners_found;
    }

    fn check_fork() {

    }

    fn is_corner(&self, q: &i32, r: &i32) -> bool {
        let mut is_corner: bool = false;

        for corner in &self.corners {
            if (*q, *r) == *corner {
                is_corner = true;
            }
        };

        return is_corner;
    }

    fn get_hex_owner(&self, q: &i32, r: &i32) -> HexOwner {
        // WARNING: this might cause issues. Probably will fix later on. 
        if !self.board.is_hex_in_bounds(*q, *r) { return HexOwner::None}
        self.board.state.get(&(*q, *r)).unwrap().owner
    }
}