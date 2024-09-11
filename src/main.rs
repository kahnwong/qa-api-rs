mod llm;

use crate::llm::call_llm;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

const MODE: &str = dotenv!("MODE");
const QA_API_KEY: &str = dotenv!("QA_API_KEY");

#[derive(Serialize, Deserialize)]
struct SubmitRequest {
    request_id: String,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubmitResponse {
    request_id: String,
    query: String,
    response: String,
}

#[get("/")]
async fn root() -> impl Responder {
    "Welcome to qa-api-rs"
}

#[post("/submit")]
async fn submit(body: web::Bytes) -> Result<HttpResponse, Error> {
    log::info!("Info!!!");

    let r = serde_json::from_slice::<SubmitRequest>(&body)?;
    let answer = call_llm(&r.query);

    let response = SubmitResponse {
        request_id: r.request_id,
        query: (&r.query).to_string(),
        response: answer,
    };

    log::info!("{:?}", response);

    let result = Ok(HttpResponse::Ok().json(response));
    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    tracing_subscriber::fmt().json().init();

    // set hostname
    let listen_address;
    match MODE {
        "production" => listen_address = "0.0.0.0",
        "development" => listen_address = "127.0.0.1",
        _ => {
            log::error!("Please specify env MODE");
            std::process::exit(1);
        }
    }

    // init server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(root)
            .service(submit)
    })
    .bind((listen_address, 8080))?
    .run()
    .await
}
