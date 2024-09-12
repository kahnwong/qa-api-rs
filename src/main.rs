mod llm;
use crate::llm::llm_call;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

const MODE: &str = dotenv!("MODE");
// const QA_API_KEY: &str = dotenv!("QA_API_KEY");

#[derive(Deserialize)]
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

async fn root() -> &'static str {
    "Welcome to qa-api-rs"
}

async fn submit(Json(payload): Json<SubmitRequest>) -> (StatusCode, Json<SubmitResponse>) {
    // get answer
    let answer = llm_call(&payload.query).await.unwrap();

    // return response
    let response = SubmitResponse {
        request_id: payload.request_id,
        query: (&payload.query).to_string(),
        response: answer,
    };

    log::info!("{}", serde_json::to_string(&response).unwrap());

    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    // init logger
    tracing_subscriber::fmt().json().init();

    // set hostname
    let listen_host;
    match MODE {
        "production" => listen_host = "0.0.0.0",
        "development" => listen_host = "127.0.0.1",
        _ => {
            log::error!("Please specify env MODE");
            std::process::exit(1);
        }
    }
    let listen_address = format!("{}:3000", listen_host);

    // routes
    let app = Router::new()
        .route("/", get(root))
        .route("/submit", post(submit));

    // init server
    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
