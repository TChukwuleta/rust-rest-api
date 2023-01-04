extern crate log;
mod user;
mod api_error;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;
use dotenv::dotenv;
use log::info;
use std::env;
use user::routes::init_routes;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let redis_port = env::var("REDIS_PORT").expect("Redis environment port not found");
    let redis_host = env::var("REDIS_HOST").expect("Redis environment host not found");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| 
        App::new()
        .configure(init_routes)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host environment variable not defined");
            let port = env::var("PORT").expect("Port environment variable not defined");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    info!("Starting server");
    server.run().await
}


