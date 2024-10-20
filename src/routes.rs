use crate::llm::get_answer;
use crate::Config;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubmitRequest {
    request_id: String,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitResponse {
    request_id: String,
    query: String,
    response: String,
}

pub async fn root() -> &'static str {
    "Welcome to qa-api-rs"
}

pub async fn submit(
    State(config): State<Config>,
    Json(payload): Json<SubmitRequest>,
) -> (StatusCode, Json<SubmitResponse>) {
    tracing::info!("{}", serde_json::to_string(&payload).unwrap());

    // get answer
    let answer = get_answer(&config.google_ai_api_key, &payload.query).await;

    // return response
    let response = SubmitResponse {
        request_id: payload.request_id,
        query: payload.query,
        response: answer,
    };

    tracing::info!("{}", serde_json::to_string(&response).unwrap());

    (StatusCode::OK, Json(response))
}
