use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> Self {
        ApiError { status_code, message }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.message.as_str())
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        
    }
}