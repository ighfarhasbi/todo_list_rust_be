use axum::http::StatusCode;
use bcrypt::{hash, verify};


pub fn hash_password(password: &String) -> Result<String, StatusCode> {
    hash(password, 12).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(password: &String, hash: &str) -> Result<bool, StatusCode> {
    verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}