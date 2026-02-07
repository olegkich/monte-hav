use std::time::Instant;
use actix_web::{post, web, HttpResponse, Responder};
use crate::{board::BoardState, mcts::MCTS, server_data::{GameMoveRequest, GameMoveResponse}, types::HexOwner};

#[post("/get-move")]
async fn get_move(req: web::Json<GameMoveRequest>) -> impl Responder {
    println!("Request received. ");

    let board = parse_board_state(&req.board_state, req.board_size);
    
    let start = Instant::now();
    let mcts = MCTS::new();
    let best_move = mcts.run_parallel(board, 12, 5000);
    let duration = start.elapsed().as_secs();
    
    println!("AI played a move: {:?};\n sending to client...", best_move);

    HttpResponse::Ok().json(GameMoveResponse {
        best_move,
        time_ms: duration,
    })
}

fn parse_board_state(hex_data: &std::collections::HashMap<String, crate::server_data::HexData>, board_size: i8) -> BoardState {
    let mut board = BoardState::new(board_size);
    
    for (_key, hex) in hex_data {
        let owner = match hex.owner.as_str() {
            "P1" => HexOwner::P1,
            "P2" => HexOwner::P2,
            _ => HexOwner::None,
        };
        
        board.set_hex(hex.q, hex.r, owner);
    }
    
    board
}