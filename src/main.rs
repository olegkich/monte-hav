
mod types;
mod board;
mod display;
mod game;
mod win_detector;
mod mcts;
mod server;
mod game_service; 
mod server_data;

use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut board = BoardState::new(5);
    // board.start_game_ai_vs_ai(5000, Some(6));

    let server = Server::new(4000);
    server.start().await
}

