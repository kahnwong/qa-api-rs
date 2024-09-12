mod llm;
mod routes;

use crate::routes::{root, submit};
use axum::{
    routing::{get, post},
    Router,
};
// use dotenv_codegen::dotenv;
use listenfd::ListenFd;
use tokio::net::TcpListener;

// const QA_API_KEY: &str = dotenv!("QA_API_KEY");

#[tokio::main]
async fn main() {
    // init logger
    tracing_subscriber::fmt().json().init();

    // routes
    let app = Router::new()
        .route("/", get(root))
        .route("/submit", post(submit));

    // init server
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("0.0.0.0:3000").await.unwrap(),
    };

    axum::serve(listener, app).await.unwrap();
}
