
use crate::{board::{self, BoardState}, win_detector};
use rand::Rng;

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
            self.expand(node_index);
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

    fn expand(&mut self, node_index: usize) {

        if let Some(node) = self.nodes.get_mut(node_index) {
            let moves = node.state.legal_moves();


        };
    }

    fn calculate_uct(&self, node: &Node, parent_visits: u32) -> f32 {
        if node.visits == 0 {
            // to encourage exploring unvisited nodes
            return f32::INFINITY;
        }

        let w_i = node.total_reward;
        let n_i = node.visits as f32;
        let c = self.exploration_constant;

        (w_i / n_i) + c * (((parent_visits as f32).ln() / n_i).sqrt())
    }

}
