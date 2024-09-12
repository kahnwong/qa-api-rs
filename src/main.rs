mod llm;
mod routes;
use crate::routes::{root, submit};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use dotenv_codegen::dotenv;
use listenfd::ListenFd;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

const QA_API_KEY: &str = dotenv!("QA_API_KEY");

#[tokio::main]
async fn main() {
    // init logger
    tracing_subscriber::fmt().json().init();

    // set rate limiting
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);

    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        tracing::info!("rate limiting storage size: {}", governor_limiter.len());
        governor_limiter.retain_recent();
    });

    // routes
    let base = Router::new().route("/", get(root));

    let qa = Router::new()
        .route("/submit", post(submit))
        .layer(middleware::from_fn(auth_middleware))
        .layer(GovernorLayer {
            config: governor_conf,
        });

    let app = base.merge(qa);

    // init server
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("0.0.0.0:3000").await.unwrap(),
    };

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn auth_middleware(req: Request, next: Next) -> Response {
    if let Some(api_key) = req.headers().get("x-api-key") {
        println!("{:?}", api_key);

        if api_key == QA_API_KEY {
            return next.run(req).await;
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
