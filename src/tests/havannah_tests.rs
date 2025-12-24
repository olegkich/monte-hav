use board::{BoardState, Hex, HexOwner, Player};
use win_detector::WinDetector;
use mcts::MCTS;

// --- WIN DETECTOR TESTS ---

#[test]
fn test_ring_detection() {
    let mut board = BoardState::new(4);
    
    // Create a small ring around (0,0)
    let ring_moves = vec![
        (1, -1), (1, 0), (0, 1), 
        (-1, 1), (-1, 0), (0, -1)
    ];

    for (q, r) in ring_moves {
        board.state.insert((q, r), Hex { q, r, owner: HexOwner::P1 });
    }

    // Place an empty or enemy hex inside to ensure it's a real ring
    board.state.insert((0, 0), Hex { q: 0, r: 0, owner: HexOwner::P2 });

    let detector = WinDetector::from_board(&board);
    assert!(detector.check_ring(&Player::P1), "Should detect a ring for P1");
    assert!(!detector.check_ring(&Player::P2), "Should NOT detect a ring for P2");
}

#[test]
fn test_bridge_detection() {
    let mut board = BoardState::new(3); // Small board (radius 3)
    // Corners at size 3 are usually at dist 2 from center? 
    // Let's trace a path between two corners.
    // Corners: (0, -2) and (0, 2)
    
    let bridge_moves = vec![
        (0, -2), // Corner
        (0, -1),
        (0, 0),
        (0, 1),
        (0, 2)   // Corner
    ];

    for (q, r) in bridge_moves {
        board.state.insert((q, r), Hex { q, r, owner: HexOwner::P1 });
    }

    let detector = WinDetector::from_board(&board);
    // Note: detector.run calls check_bridge internally
    assert!(detector.run(&Player::P1), "Should detect a bridge for P1");
}

#[test]
fn test_fork_detection() {
    let mut board = BoardState::new(3);
    
    // Connect center to 3 different edges
    // Edges are roughly at dist 2.
    let fork_moves = vec![
        (0,0), // Center
        // Arm 1 (Top Left Edge)
        (0, -1), (0, -2), 
        // Arm 2 (Bottom Right Edge)
        (0, 1), (0, 2),
        // Arm 3 (Top Right Edge)
        (1, -1), (2, -2) 
    ];

    for (q, r) in fork_moves {
        board.state.insert((q, r), Hex { q, r, owner: HexOwner::P1 });
    }

    let detector = WinDetector::from_board(&board);
    assert!(detector.run(&Player::P1), "Should detect a fork for P1");
}

// --- LOGIC / API TESTS ---

#[test]
fn test_out_of_bounds() {
    let mut board = BoardState::new(3);
    // (10, 10) is definitely out of bounds for size 3
    let result = board.apply_move((10, 10));
    assert!(result.is_err(), "Should return error for OOB move");
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
    board.apply_move((0, 0)).unwrap(); // P1 moves
    
    let result = board.apply_move((0, 0)); // P2 tries same spot
    assert!(result.is_err(), "Should not allow moving on occupied tile");
    assert_eq!(board.turn, Player::P2, "Turn should not change on invalid move");
}

// --- AI BEHAVIOR TESTS ---

#[test]
fn test_mcts_finds_immediate_win_in_1_ply() {
    // This is a "Mate in 1" puzzle
    let mut board = BoardState::new(3);
    
    // Setup a board where P1 can win immediately by completing a bridge
    // Path: (0, -2) -> (0, -1) -> (0, 0) -> (0, 1) -> [TARGET]
    // Target is (0, 2)
    let setup_moves = vec![(0, -2), (0, -1), (0, 0), (0, 1)];
    
    for (q, r) in setup_moves {
        // We manually insert to keep it P1's turn
        board.state.insert((q, r), Hex { q, r, owner: HexOwner::P1 });
    }

    let mut mcts = MCTS::new();
    // Run enough iterations to ensure it sees the win
    // Since it's a direct win, even 50 iters should catch it if logic is correct
    // because the winning node will have infinite/max reward.
    let best_move = mcts.run_parallel(board, 1, 500); 

    assert_eq!(best_move, (0, 2), "AI should find the immediate winning move (0, 2)");
}

#[test]
fn test_mcts_blocks_immediate_loss() {
    // This is a "Mate in 1" for the opponent. P1 must block.
    let mut board = BoardState::new(3);
    
    // P2 is threatening a win at (0, 2)
    // We insert P2 pieces manually
    let threat_moves = vec![(0, -2), (0, -1), (0, 0), (0, 1)];
    for (q, r) in threat_moves {
        board.state.insert((q, r), Hex { q, r, owner: HexOwner::P2 });
    }

    // It is P1's turn. P1 MUST play (0, 2) to stop the bridge.
    // (Assuming P1 knows how to block)
    
    let mut mcts = MCTS::new();
    let best_move = mcts.run_parallel(board, 1, 500);

    assert_eq!(best_move, (0, 2), "AI should block the opponent's winning move");
}