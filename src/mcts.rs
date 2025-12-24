
use core::panic;
use std::collections::HashMap;

use crate::{board::{self, BoardState, Hex, Player}, win_detector};
use rand::{Rng, rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// TODO: (MAX PRIORITY) add an instant win check because the AI misses winning moves on low iters.
#[derive(Debug)]
struct Node {
    state: BoardState,
    parent_index: Option<usize>,
    children: Vec<usize>,
    visits: u32,
    total_reward: f32,
    uct: f32,
    is_terminal: bool,
    player_to_move: board::Player,
    last_move: Option<(i32, i32)>
}

impl Node {
    pub fn new(state: BoardState, parent_index: Option<usize>, last_move: Option<(i32, i32)>) -> Self {
        let is_terminal = is_terminal(&state);
        let player_to_move = state.turn;

        Self  {
            state,
            parent_index,
            children: vec![],
            visits: 0,
            total_reward: 0.0,
            uct: 0.0,
            is_terminal,
            player_to_move: player_to_move,
            last_move
        }
    }
}

fn is_terminal(state: &BoardState) -> bool {
        let win_detector = win_detector::WinDetector::from_board(state);
    
        return win_detector.run(&board::Player::P1) || win_detector.run(&board::Player::P2)
    }

pub struct MCTS {
    nodes: Vec<Node>,
    exploration_constant: f32,
    max_iter: i32,
}

impl MCTS {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            exploration_constant: (2.0 as f32).sqrt(),
            max_iter: 1000
        }
    }

    pub fn run(&mut self, start_state: BoardState) -> (i32, i32) {
        let root_index = self.search(start_state);
        
        return self.best_move(root_index);
    }

    
    pub fn run_parallel(&self, start_state: BoardState, threads: usize, iters: usize) -> (i32, i32) {
        // each thread builds its own smaller tree
        // for each thread return a hashmap which contains { move: visit count }  
        let results: Vec<HashMap<(i32, i32), u32>> = (0..threads).into_par_iter().map(|_| {
            
            let mut local_mcts = MCTS {
                nodes: vec![],
                exploration_constant: self.exploration_constant,
                max_iter: iters as i32,
            };


            let root_index = local_mcts.search(start_state.clone());

            let root = &local_mcts.nodes[root_index];

            let mut map = HashMap::new();

            for &child_index in &root.children {
                let child = &local_mcts.nodes[child_index];

                if let Some(m) = child.last_move {
                    *map.entry(m).or_insert(0) += child.visits;
                }
            }
            map
        }).collect();

        // merge
        let mut global_visits = HashMap::new();
        
        for r in results {
            for (m, v) in r {
                *global_visits.entry(m).or_insert(0) += v;
            }
        }

        let best = global_visits.into_iter().max_by_key(|&(_, v)| v).unwrap().0;
        best
    }

    fn best_move(&self, root_index: usize) -> (i32, i32){
        
        // WARNING: unwrap
        let root = self.nodes.get(root_index).unwrap();

        if root.children.is_empty() {
            panic!("root node has no children after search()");
        };

        let mut best_visits = 0;
        let mut best_index = root.children[0];

        for child_index in &root.children {
            let child = self.nodes.get(*child_index).unwrap();

            if child.visits > best_visits {
                best_visits = child.visits;
                best_index = *child_index;
            }
        };

        println!("found best move with {} visits", best_visits);

        let best_move = self.nodes.get(best_index).unwrap().last_move;

        match best_move {
            Some(m) => return m,
            None => panic!("no best move found") 
        };

        
    }

    fn search(&mut self, start_state: BoardState) -> usize {
      
        if is_terminal(&start_state) {
            let root_index = self.nodes.len();
            self.nodes.push(Node::new(start_state, None, None));
            return root_index;
        }

        let root_index = self.nodes.len();
        self.nodes.push(Node::new(start_state, None, None));

        for _ in 0..self.max_iter {
            let node_index = self.select(root_index);

            if self.nodes[node_index].is_terminal {
                let reward = self.simulate(node_index);
                self.back_propagation(reward, node_index);
                continue;
            }

            let expanded_index = self.expand(node_index);
            let reward = self.simulate(expanded_index);

            self.back_propagation(reward, expanded_index);
        }

        println!("looked through {} moves", self.nodes.len());

        return root_index
        
    }

    fn select(&self, start_index: usize) -> usize {
        let node = &self.nodes[start_index];

        if node.children.is_empty() {
            return start_index;
        }

        // WARNING: (MILD) index 0 is unsafe but with NEG_INFINITY UCT the first child picked should always overwrite it
        let (mut best_uct,mut best_index): (f32, usize) = (f32::NEG_INFINITY, 0);

        for index in &node.children {
            if let Some(child_node) = self.nodes.get(*index) {
                let uct = self.calculate_uct(child_node, node.visits);
                
                if (uct > best_uct) {
                    best_uct = uct;
                    best_index = *index;
                }
            };   
        };

        return self.select(best_index);
    }

    fn expand(&mut self, node_index: usize) -> usize {


        let moves = {
            let node = &mut self.nodes[node_index];
            node.state.legal_moves()
        };

        if moves.is_empty() {
            panic!("no legal moves available")
        }

        // Create ALL children at once
        for &move_coords in &moves {
            let mut new_state = self.nodes[node_index].state.clone();
            new_state.apply_move(move_coords).unwrap();
            
            let new_node = Node::new(new_state, Some(node_index), Some(move_coords));
            let new_index = self.nodes.len();
            
            self.nodes.push(new_node);
            self.nodes[node_index].children.push(new_index);
        }

        let children = &self.nodes[node_index].children;
        let random_idx = self.get_random_move_index(children.len());
        children[random_idx]
       
    }

    fn simulate(&self, start_index: usize) -> f32  {
        if let Some(node) = self.nodes.get(start_index) {

            // in case expanded_node is already terminal
            if node.is_terminal {
                let winner = node.state.get_winner();
                return match winner {
                    Some(p) if p == node.player_to_move => -1.0,
                    Some(_) => 1.0,
                    None => 0.0,
                };
            }


            let mut board = node.state.clone();
            
            let mut n_moves = 0;

            while !board.is_terminal() {
                let moves = board.legal_moves();
                let r_index = self.get_random_move_index(moves.len());
                let r_move = moves[r_index];
                board.apply_move(r_move).unwrap();

                n_moves += 1;
            };

            let winner = board.get_winner();

            // since board contains the turn after the node was expanded.
            let last_player = match node.player_to_move {
                Player::P1 => Player::P2,
                Player::P2 => Player::P1,
            };

            return match winner {
                Some(p) if p == last_player => 1.0,
                Some(_) => -1.0,
                None => 0.0,
            };
        }   

        else {
            panic!("no node to simulate.");
        }
    }

    fn back_propagation(&mut self, mut reward: f32, expanded_index: usize) {
        let mut current_index = Some(expanded_index);

        while let Some(index) = current_index {
            let node = &mut self.nodes[index];
            node.visits += 1;
            node.total_reward += reward;

            reward = -reward;

            current_index = node.parent_index;
        }
    }

    fn calculate_uct(&self, node: &Node, parent_visits: u32) -> f32 {
        if node.visits == 0 {
            return f32::INFINITY;
        }
        if parent_visits == 0 {
            return 0.0;
        }

        let w_i = node.total_reward;
        let n_i = node.visits as f32;
        let c = self.exploration_constant;

        (w_i / n_i) + c * (((parent_visits as f32).ln() / n_i).sqrt())
        }


    fn get_random_move_index(&self, max: usize) -> usize {
        rand::rng().random_range(0..max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{BoardState, Hex, HexOwner, Player};

    #[test]
    // The root node (the state from which a move will be made)
    // is considered players 2s move, even though player 1 is about to move
    // So in case of a sequence: (Win in 1 for P1) -> (P1 makes the winning move, state becomes terminal)
    // The first node should obviously have NEG_INFINIGTY reward since it's a loss for P2 
    // Writing this down for future, the backprop sign flipping confuses me
    fn test_negamax_reward_attribution() {
        // SCENARIO: P1 is about to win.
        // force MCTS to expand the winning node and check the rewards.
        
        let mut board = BoardState::new(3); // Small board
        // Set up a P1 Bridge 
        board.state.insert((0, -2), Hex { q: 0, r: -2, owner: HexOwner::P1 });
        board.state.insert((1, -2), Hex { q: 1, r: -2, owner: HexOwner::P1 });
        board.state.insert((0, 0),  Hex { q: 0, r: 0,  owner: HexOwner::P1 });
        board.state.insert((0, 1),  Hex { q: 0, r: 1,  owner: HexOwner::P1 });
        
        let mut mcts = MCTS::new();
        
        let root_idx = mcts.search(board.clone());
        let root_node = &mcts.nodes[root_idx];
        assert_eq!(root_node.player_to_move, Player::P1);

        let mut winning_state = board.clone();
        winning_state.apply_move((2, -2)).unwrap();
        
        assert!(winning_state.is_terminal());
        assert_eq!(winning_state.get_winner(), Some(Player::P1));

        let child_node = Node::new(winning_state, Some(root_idx), Some((0, 2)));
        mcts.nodes.push(child_node);
        let child_idx = mcts.nodes.len() - 1;
        mcts.nodes[root_idx].children.push(child_idx);

        let reward = mcts.simulate(child_idx);
        
        // Simulate should return +1 because the move-maker - P1 won.
        assert_eq!(reward, 1.0, "Simulate should return +1 for a win by the previous player");

        mcts.back_propagation(reward, child_idx);

        let child_node = &mcts.nodes[child_idx];
        let root_node = &mcts.nodes[root_idx];

        assert!(child_node.total_reward > 0.0, "Winning move must have positive value!");
        assert!(root_node.total_reward < 0.0, "Root value should be inverted relative to child");
    }

    #[test]
    fn test_mcts_finds_immediate_win_in_1_ply() {
        let mut board = BoardState::new(3);

        // build a bridge
        let setup_moves = vec![(0, -2), (0, -1), (0, 0), (0, 1)];
        
        for (q, r) in setup_moves {
            board.state.insert((q, r), Hex { q, r, owner: HexOwner::P1 });
        }

        let mcts = MCTS::new();
        // Run enough iterations to ensure it sees the win
        // Since it's a direct win, even 50 iters should find it if logic is correct
        // because the winning node will have infinite/max reward.
        let best_move = mcts.run_parallel(board, 6, 1000); 

        println!("AI's best move: {:?}", best_move);

        assert_eq!(best_move, (0, 2), "AI should find the immediate winning move (0, 2)");
    }

    #[test]
    fn test_mcts_blocks_immediate_loss() {
        let mut board = BoardState::new(3);
        
        // again, a bridge
        let threat_moves = vec![(0, -2), (0, -1), (0, 0), (0, 1)];

        for (q, r) in threat_moves {
            board.state.insert((q, r), Hex { q, r, owner: HexOwner::P2 });
        }

        // It is P1s turn
        // P1 SHOULD play (0, 2) to stop the bridge.
        
        let mcts = MCTS::new();
        let best_move = mcts.run_parallel(board, 6, 1000);

        assert_eq!(best_move, (0, 2), "AI should block the opponent's winning move");
    }
}