use std::sync::{Arc, Mutex};

use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use duckdb::{params, Connection};

use crate::{helper::hash_pass::hash_password, models::{general_response::ResponseModel, users::{ReqAddUser, ResAddUser}}};


pub async fn get_user() -> String {
    "from user fn".to_owned()
}

pub async fn add_user(conn: Extension<Arc<Mutex<Connection>>>, req_add_user: Json<ReqAddUser>) -> Result<Response, (StatusCode, String)> {
    let pass = hash_password(&req_add_user.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Gagal hashing password".to_owned()))?;
    let conn = conn.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock connection".to_string()))?;
    let mut stmt = conn.prepare("INSERT INTO user (no_hp, nama_depan, nama_belakang, password, otp) VALUES (?, ?, ?, ?, ?)")
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    stmt.execute(params![req_add_user.no_hp, req_add_user.nama_depan, req_add_user.nama_belakang, pass, req_add_user.otp])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let val = ResAddUser {
        nama_depan: req_add_user.nama_depan.to_string(),
    };
    let result = ResponseModel {
        kode: StatusCode::CREATED.to_string(),
        message: "Sukses".to_string(),
        data: Some(val),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}