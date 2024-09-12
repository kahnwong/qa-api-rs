mod llm;
mod routes;

use crate::routes::{root, submit};

use axum::{
    routing::{get, post},
    Router,
};
use dotenv_codegen::dotenv;

const MODE: &str = dotenv!("MODE");
// const QA_API_KEY: &str = dotenv!("QA_API_KEY");

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
