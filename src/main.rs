use actix_web::{get, web, App, HttpServer, Responder};
use dotenv_codegen::dotenv;
use env_logger::Env;
// use log::{debug, error, info, log_enabled, Level};
use actix_web::middleware::Logger;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("{}", dotenv!("MODE"));
    // info!("Info!!!");
    format!("FFF {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
