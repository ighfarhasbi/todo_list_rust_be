use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqLogin {
    pub no_hp: String,
    pub password: String
}

#[derive(Serialize)]
pub struct ResLoginUser {
    pub nama_depan: String,
    pub token: String,
}