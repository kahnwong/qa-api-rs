use actix_web::{get, web, App, HttpServer, Responder};
use dotenv_codegen::dotenv;
use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("{}", dotenv!("MODE"));
    info!("Info!!!");
    format!("FFF {name}!")
}

#[get("/foo/{i}")]
async fn foo(i: web::Path<String>) -> impl Responder {
    format!("AAA {i}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new().service(greet).service(foo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
