use crate::llm::get_answer;
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

pub async fn submit(Json(payload): Json<SubmitRequest>) -> (StatusCode, Json<SubmitResponse>) {
    log::info!("{}", serde_json::to_string(&payload).unwrap());

    // get answer
    let answer = get_answer(&payload.query).await;

    // return response
    let response = SubmitResponse {
        request_id: payload.request_id,
        query: (&payload.query).to_string(),
        response: answer,
    };

    log::info!("{}", serde_json::to_string(&response).unwrap());

    (StatusCode::OK, Json(response))
}
