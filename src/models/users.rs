use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqAddUser {
    pub no_hp: String,
    pub nama_depan: String,
    pub nama_belakang: String,
    pub password: String,
    pub otp: String,
}

#[derive(Serialize)]
pub struct ResAddUser {
    pub nama_depan: String,
}