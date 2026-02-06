use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub struct Server {
    port: u16,
}   

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}