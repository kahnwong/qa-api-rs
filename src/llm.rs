use dotenv_codegen::dotenv;
use reqwest::{Client, StatusCode, Response};
use serde_json::from_str;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;
use std::ops::Index;

const GOOGLE_AI_API_KEY: &str = dotenv!("GOOGLE_AI_API_KEY");

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: UsageMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: i64,
    pub candidates_token_count: i64,
    pub total_token_count: i64,
}

pub async fn llm_call(query: &String) -> Result<String, Box<dyn Error>>   {
    let client = Client::new();

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", GOOGLE_AI_API_KEY);
    let payload = &serde_json::json!({
      "contents": [{
        "parts":[{"text": format!("{}", query), }]
        }]
       });

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    // parse
    let response_text = response.text().await?;
    let response_struct: GeminiResponse = from_str(&response_text)?;

    let answer = response_struct.candidates.index(0).content.parts.index(0).text.clone();
    Ok(answer)
}
