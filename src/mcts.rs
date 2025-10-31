
use crate::{board::{self, BoardState}, win_detector};

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

    fn search(&self, start_state: BoardState) {
        let root_node = Node::new(start_state, None);

        
    }

    fn select() {

    }

    fn calculate_uct(&self, node: &Node, parent_visits: f32) -> f32 {
        if node.visits == 0 {
            // to encourage exploring unvisited nodes
            return f32::INFINITY;
        }

        let w_i = node.total_reward;
        let n_i = node.visits as f32;
        let c = self.exploration_constant;

        (w_i / n_i) + c * ((parent_visits.ln() / n_i).sqrt())
    }

    fn get_node_by_index(&self, i: Option<usize>) -> Option<&Node> {
        match i {
            None => None,
            Some(index) => self.nodes.get(index),
        }
    }
}
