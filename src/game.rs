/// Game orchestration and runner logic.
/// Handles game loops for human vs AI and AI vs AI matches.

use std::io::stdin;
use std::time::Instant;

use crate::board::BoardState;
use crate::display::BoardDisplay;
use crate::mcts;
use crate::win_detector::WinDetector;

pub trait GameRunner {
    fn start_game(&mut self);
    fn start_game_vs_ai(&mut self);
    fn start_game_ai_vs_ai(&mut self, iters: usize, threads: Option<usize>);
}

impl GameRunner for BoardState {
    fn start_game_ai_vs_ai(&mut self, iters: usize, threads: Option<usize>) {
        loop {
            self.print_state_pretty();
            
            let mut ai1 = mcts::MCTS::new();

            let start = Instant::now();

            let best_move1;

            match threads {
                None => best_move1 = ai1.run(self.clone()),
                Some(threads) => best_move1 = ai1.run_parallel(self.clone(), threads, iters)
            }
    
            let duration = start.elapsed();

            println!("AI 1 plays: ({}, {}) in {:.2?}", best_move1.0, best_move1.1, duration);

            self.apply_move(best_move1).unwrap();

            if self.is_terminal() {
                println!("AI 1 won");
                break;
            }

            let mut ai2 = mcts::MCTS::new();

            let best_move2;
    
            match threads {
                None => best_move2 = ai2.run(self.clone()),
                Some(threads) => best_move2 = ai2.run_parallel(self.clone(), threads, iters)
            }

            println!("AI 2 plays: ({}, {}) in {:.2?}", best_move2.0, best_move2.1, duration);

            self.apply_move(best_move2).unwrap();

            if self.is_terminal() {
                println!("AI 2 won");
                break;
            }
        }

        self.print_state_pretty();
    }

    fn start_game_vs_ai(&mut self) {
        loop {
            print!("\n\n\n");

            self.print_state_pretty();

            let _win_detector = WinDetector::from_board(self);

            println!("Enter move with format: q r");

            let mut input: String = String::new();

            stdin().read_line(&mut input).unwrap();

            if input == "x" { break };

            let chars: Vec<&str> = input.split_whitespace().collect();

            let q: i32 = chars[0].trim().parse().unwrap();
            let r: i32 = chars[1].trim().parse().unwrap();

            self.apply_move((q, r)).unwrap();

            if self.is_terminal() {
                println!("player {:?} won", self.turn);
                return;
            }

            print!("AI is thinking...");

            let mut ai = mcts::MCTS::new();

            let best_move = ai.run(self.clone());
    
            println!("AI plays: ({}, {})", best_move.0, best_move.1);

            self.apply_move(best_move).unwrap();

            if self.is_terminal() {
                println!("player {:?} won", self.turn);
                return;
            }
        }
    }

    fn start_game(&mut self) {
        loop {
            print!("\n\n\n");

            self.print_state_pretty();

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
}
