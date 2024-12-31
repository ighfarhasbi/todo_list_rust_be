mod handler;
mod user;
mod login;
mod todolist;

use std::sync::{Arc, Mutex};
use duckdb::Connection;
use axum::{middleware, routing::{get, post}, Extension, Router};
use login::logout_user;

use crate::middleware::guard::guard_route;

pub fn router(conn: Arc<Mutex<Connection>>) -> Router {
    Router::new()
    .route("/api/v1/todolist", get(todolist::get_data))
    .route("/api/v1/todolist", post(todolist::add_data))
    .route("/api/v1/todolist/:id", post(todolist::delete_data))
    .route("/api/v1/todolist/:id", post(todolist::update_data))
    .route("/api/v1/logout", post(logout_user))
    .route_layer(middleware::from_fn(guard_route))
    .route("/", get(handler::hello))
    .route("/api/v1/user", get(user::get_user))
    .route("/api/v1/user", post(user::add_user))
    .route("/api/v1/login", get(login::login))
    .layer(Extension(conn))
}