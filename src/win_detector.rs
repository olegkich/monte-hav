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
        let edges = Self::initialize_edges(board.board_size);

        Self { board, corners, edges }
    }

    pub fn run(&self, player: &Player) -> bool {
        return self.check_bridge(player) || self.check_fork(player) || self.check_ring(player);
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

        let max_qr: i32 = (board_size) as i32 - 1;

        return vec![
            (-max_qr, 0),
            (-max_qr, max_qr),
            (0, -max_qr),
            (0, max_qr),
            (max_qr, -max_qr),
            (max_qr, 0)
        ];        
    }

    fn initialize_edges(board_size: i8) -> Vec<(i32, i32)> {
        let mut edges: Vec<(i32, i32)> = Vec::new();
        let b = board_size as i32;
        let r = b - 1;

        // L, R
        for i in 1..r { // skip corners
            let left = (-r, i);
            let right = (r, i - r);
            edges.push(left);
            edges.push(right);
        }

        // UR, LB
        for i in 1..r { // skip corners
            let ur = (i, -r);
            let lb = (i - r, r);

            edges.push(ur);
            edges.push(lb);
        }

        // RB, UL
        for i in 1..r { // skip corners
            let rb = (i, r - i);
            let ul = (i - r, -i);

            edges.push(rb);
            edges.push(ul);
        }

        edges
    }

    fn edge_side(&self, q: i32, r: i32) -> Option<u8> {
        let n = self.board.board_size as i32 - 1;
        let s = -q - r;

        if q == -n { Some(0) }
        else if r == -n { Some(1) }
        else if s == -n { Some(2) }
        else if q == n { Some(3) }
        else if r == n { Some(4) }
        else if s == n { Some(5) }
        else { None }
    }

    pub fn check_ring(&self, player: &Player) -> bool {
        for ((_, _), hex) in &self.board.state {
            let owner = hex.owner;

            if owner == HexOwner::None || owner != HexOwner::from(player) {
                if !self.can_empty_cell_escape(hex.q, hex.r, player) {
                    return true; 
                }
            }
        }

        false
    }

    fn can_empty_cell_escape(&self, start_q: i32, start_r: i32, player: &Player) -> bool {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        visited.insert((start_q, start_r));
        queue.push_back((start_q, start_r));

        while let Some((q, r)) = queue.pop_front() {
            if self.is_on_board_boundary(q, r) {
                return true;
            }

            for (nq, nr) in self.get_neighbours(&q, &r) {
                if visited.contains(&(nq, nr)) {
                    continue;
                }

                let owner = self.get_hex_owner(&nq, &nr);

               
                if owner != HexOwner::from(player) {
                    visited.insert((nq, nr));
                    queue.push_back((nq, nr));
                }
            }
        }

        false 
    }

    fn is_on_board_boundary(&self, q: i32, r: i32) -> bool {
        let s = -q - r;
        let n = self.board.board_size as i32 - 1;
        q.abs() == n || r.abs() == n || s.abs() == n
    }


    fn check_bridge(&self, player: &Player) -> bool {
        for ((_, _), hex) in &self.board.state {

            if hex.owner != HexOwner::from(player) { continue; };

            let (corners, _) = self.find_connection(&hex.q, &hex.r, &player);
            
            if corners.len() >= 2 { return true };
        };

        return false;
    }
    
    fn check_fork(&self, player: &Player) -> bool {
         for ((_, _), hex) in &self.board.state {

            if hex.owner != HexOwner::from(player) { continue; };

            let (_, edges) = self.find_connection(&hex.q, &hex.r, &player);
            
            if edges.len() >= 3 { return true };
        };

        return false;
    }

    fn find_connection(&self, start_q: &i32, start_r: &i32, player: &Player) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut corners_found: HashSet<(i32, i32)> = HashSet::new();
        // stores only 1 edge for a side
        // TODO: rename to sides
        let mut edges_found: HashSet<(i32, i32)> = HashSet::new();

        let mut queue: VecDeque<(i32, i32)> = VecDeque::from([(*start_q, *start_r)]);

        while queue.len() > 0 {
            let (q, r) = queue.pop_front().unwrap();

            if self.is_corner(&q, &r) {
                corners_found.insert((q, r));
            }

            if let Some(side) = self.edge_side(q, r) {
                edges_found.insert((side as i32, 0));
            }

            let neighbours = self.get_neighbours(&q, &r);

            for (qn, rn) in neighbours {
                if visited.contains(&(qn, rn)) { continue; }
                
                
                if (self.get_hex_owner(&qn, &rn) == HexOwner::from(player)) {
                    visited.insert((qn, rn));
                    queue.push_front((qn, rn));
                }                
            }
        }
        return (corners_found, edges_found);
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

    fn is_edge(&self, q: &i32, r: &i32) -> bool {
        let mut is_edge: bool = false;

        for edge in &self.edges {
            if (*q, *r) == *edge {
                is_edge = true;
            }
        };

        return is_edge;
    }

    fn get_hex_owner(&self, q: &i32, r: &i32) -> HexOwner {
        // WARNING: this might cause issues. Probably will fix later on. 
        if !self.board.is_hex_in_bounds(*q, *r) { return HexOwner::None}
        self.board.state.get(&(*q, *r)).unwrap().owner
    }
}