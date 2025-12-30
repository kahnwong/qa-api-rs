use reqwest::Client;
use serde_derive::Deserialize;
use serde_json::from_str;
use std::error::Error;
use std::ops::Index;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
    // pub usage_metadata: UsageMetadata,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    // pub finish_reason: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub parts: Vec<Part>,
    // pub role: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub text: String,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UsageMetadata {
//     pub prompt_token_count: i64,
//     pub candidates_token_count: i64,
//     pub total_token_count: i64,
// }

pub async fn llm_call(
    google_ai_api_key: &String,
    query: &String,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", google_ai_api_key);

    let payload = &serde_json::json!(
        {
            "contents": [{
                "parts":[{"text": format!("{}", query)}]
            }],
            "generationConfig": {
                "maxOutputTokens": 1024,
            }
        }
    );

    let response = client.post(url).json(&payload).send().await?;

    // parse
    let response_text = response.text().await?;
    let response_struct: GeminiResponse = from_str(&response_text)?;

    let answer = response_struct
        .candidates
        .index(0)
        .content
        .parts
        .index(0)
        .text
        .clone();
    Ok(answer)
}

pub async fn get_answer(google_ai_api_key: &String, question: &String) -> String {
    let question_prompt = format!("Answer following question about data engineering: {}. Answer should be within 4 paragraphs. Please respond in Thai", question);
    let answer = llm_call(google_ai_api_key, &question_prompt).await.unwrap();

    // prevent prompt injections
    let verify_prompt = format!("Your role is to verify that the following text is a question related to data engineering: {}. Please only answer with only True or False", question);
    let verify_answer = llm_call(google_ai_api_key, &verify_prompt).await.unwrap();

    // return response
    let response = match verify_answer.trim() {
        "True" => answer,
        _ => "Error: question should be about data engineering or related topics.".to_string(),
    };
    response
}
