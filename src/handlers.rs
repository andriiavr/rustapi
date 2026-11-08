use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde_json::{Value, json};

pub async fn get_posts() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "posts": ["post1", "post2", "post3"]
    });
    Json(json_response)
}

pub async fn new_post(
    result: Result<Json<Value>, JsonRejection>,
) {
    println!("{:#?}", result)
}

pub async fn calculate() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Calc is short for calculator"
    });
    Json(json_response)
}