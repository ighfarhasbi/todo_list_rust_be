mod handler;
mod user;
mod login;

use std::sync::{Arc, Mutex};
use duckdb::Connection;
use axum::{routing::{get, post}, Extension, Router};

pub fn router(conn: Arc<Mutex<Connection>>) -> Router {
    Router::new()
    .route("/", get(handler::hello))
    .route("/api/v1/user", get(user::get_user))
    .route("/api/v1/user", post(user::add_user))
    .route("/api/v1/login", get(login::login))
    .layer(Extension(conn))
}