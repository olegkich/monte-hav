use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::game_service::get_move;

pub struct Server {
    port: u16,
}   

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let server = HttpServer::new(|| {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header();
            
            App::new()
                .wrap(cors)
                .service(get_move)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await;

        server 
    }
}