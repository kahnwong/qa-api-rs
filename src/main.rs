use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use dotenv_codegen::dotenv;
use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};

const MODE: &str = dotenv!("MODE");
const QA_API_KEY: &str = dotenv!("QA_API_KEY");

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("{}", MODE);
    info!("Info!!!");
    format!("FFF {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // set hostname
    let mut listen_address = "";
    match MODE {
        "production" => listen_address = "0.0.0.0",
        "development" => listen_address = "127.0.0.1",
        _ => {
            error!("Please specify env MODE");
            std::process::exit(1);
        }
    }

    // init server
    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
