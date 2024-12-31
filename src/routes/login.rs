use std::sync::{Arc, Mutex};

use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use duckdb::{params, Connection};
use serde::Serialize;

use crate::{helper::hash_pass::verify_password, jwt::jwt::{create_access_token, create_refresh_token}, models::{general_response::{GeneralResponse, ResponseModel}, login::{ReqLogin, ResLoginUser}}};

pub async fn login(conn: Extension<Arc<Mutex<Connection>>>, login_user: Json<ReqLogin>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failled to lock connection".to_string()))?;
    let mut stmt = conn.prepare("SELECT no_hp, nama_depan, password FROM user WHERE no_hp = ?")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    #[derive(Debug, Serialize)]
    struct ResultRow {no_hp: String, nama_depan: String, password: String}
    let rows = stmt.query_map(params![login_user.no_hp], |row| {
        Ok(ResultRow {
            no_hp: row.get(0)?,
            nama_depan: row.get(1)?,
            password: row.get(2)?,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut result_user = ResultRow {
        no_hp: "".to_string(),
        nama_depan: "".to_string(),
        password: "".to_string(),
    };

    for user in rows {
        match user {
            Ok(res) => {
                result_user.no_hp = res.no_hp;
                result_user.nama_depan = res.nama_depan;
                result_user.password =  res.password;
            }
            Err(e) => {
                return Err((StatusCode::UNAUTHORIZED, e.to_string()));
            }
        }
    }

    // println!("user = {:?}", result_user);

    // Validasi username dan password
    if result_user.password.is_empty() { // menangkap ketika username salah, sehingga result_user.password kosong
        // return Err((StatusCode::UNAUTHORIZED, "Password atau username salah".to_string()));
        return Ok((StatusCode::UNAUTHORIZED, Json(GeneralResponse {
            kode: StatusCode::UNAUTHORIZED.to_string(),
            message: "Password atau username salah".to_string(),
        })).into_response());
    }
    // menangkap jika username benar dan password salah
    if !verify_password(&login_user.password, &result_user.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? {
            // return Err((StatusCode::UNAUTHORIZED, "Password atau username salah".to_string()));
            return Ok((StatusCode::UNAUTHORIZED, Json(GeneralResponse {
                kode: StatusCode::UNAUTHORIZED.to_string(),
                message: "Password atau username salah".to_string(),
            })).into_response());
    };

    // Update token dulu
    let access_token = create_access_token().unwrap();
    let refresh_token = create_refresh_token().unwrap();
    let mut stmt = conn.prepare("INSERT OR REPLACE INTO token_user (no_hp, nama_depan, access_token, refresh_token) VALUES (?, ?, ?,?)")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    stmt.execute(params![result_user.no_hp, result_user.nama_depan, access_token, refresh_token])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ambil value nama_depan dan access_token dari table token_user
    stmt = conn.prepare("SELECT nama_depan, access_token FROM token_user WHERE no_hp =?")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let rows = stmt.query_map(params![login_user.no_hp], |row| {
        Ok(ResLoginUser {
            nama_depan: row.get(0)?,
            token: row.get(1)?,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // inisialisasi struct default
    let mut result = ResponseModel { 
        kode: StatusCode::NOT_FOUND.to_string(),
        message: "Gagal".to_string(),
        data: None
    };

    // mengubah tipe data dari var "rows" menjadi struct ResponseUser berbentuk result
    for user in rows {
        match user { // mengubah result menjadi struct ResponseUser
            Ok(user) => {
                result = ResponseModel {
                    kode: StatusCode::OK.to_string(),
                    message: "Sukses".to_string(),
                    data: Some(user) // masukan struct ResponseUser ke struct ResponseModel.data
                };
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    };

     // mengecek apakah struct ResponseModel.data ada isinya atau tidak
     match &result.data {
        Some(_) => {
            Ok((StatusCode::OK, Json(result)).into_response())
        }
        None => {
            Ok((StatusCode::NOT_FOUND, Json(result)).into_response())
        }
    }
}