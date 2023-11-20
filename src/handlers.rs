use axum::{extract,http};
use serde::Deserialize;
use sqlc::PgPool;

#[derive(Debug, Deserialize)]
pub struct CreateQuote{
    book:String,
    quote:String,
}

pub sync fn health() -> http::StatusCode {
    http::StatusCode::OK
 }
 
pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,)->http::StatusCode{
        println!("Received request to create quote: {:?}", payload);
        http::StatusCode::CREATED
    }