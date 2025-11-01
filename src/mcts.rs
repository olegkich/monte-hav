
use core::panic;

use crate::{board::{self, BoardState}, win_detector};
use rand::{Rng, rng};

struct Node {
    state: BoardState,
    parent_index: Option<usize>,
    children: Vec<usize>,
    visits: u32,
    total_reward: f32,
    uct: f32,
    is_terminal: bool,
    player_to_move: board::Player,
}

impl Node {
    pub fn new(state: BoardState, parent_index: Option<usize>) -> Self {
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
            player_to_move: player_to_move
        }
    }
}

fn is_terminal(state: &BoardState) -> bool {
        let win_detector = win_detector::WinDetector::from_board(state);
    
        return win_detector.run(&board::Player::P1) || win_detector.run(&board::Player::P2)
    }

struct MCTS {
    nodes: Vec<Node>,
    exploration_constant: f32,
    max_iter: i32,
}

impl MCTS {
    fn new() -> Self {
        Self {
            nodes: vec![],
            exploration_constant: (2.0 as f32).sqrt(),
            max_iter: 1000
        }
    }

    fn search(&mut self, start_state: BoardState) {
      
        let root_index = self.nodes.len();
        self.nodes.push(Node::new(start_state, None));

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

        let r_index = self.get_random_move_index(moves.len());
        let mut new_state = self.nodes[node_index].state.clone();
    

        // WARNING: no checking for now, used unwrap! (assumes it will always find a legal move which is risky)
        new_state.apply_move(moves[r_index]).unwrap();
    

        let new_node = Node::new(new_state, Some(node_index));

        // get index before push so it's less by 1
        let new_index = self.nodes.len();

        self.nodes.push(new_node);

        self.nodes[node_index].children.push(new_index);

        return new_index
       
    }

    fn simulate(&self, start_index: usize) -> f32  {
        if let Some(node) = self.nodes.get(start_index) {
            let mut board = node.state.clone();
            
            while !board.is_terminal() {
                let moves = board.legal_moves();

                let r_index = self.get_random_move_index(moves.len());

                let r_move = moves[r_index];

                 // WARNING: no checking for now, used unwrap! (assumes it will always find a legal move which is risky)
                board.apply_move(r_move).unwrap();
            };

            let winner = board.get_winner();

            return match winner {
                Some(p) if p == self.nodes[start_index].player_to_move => 1.0,
                Some(_) => -1.0,
                None => 0.0,
            }
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
