use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseModel<T> {
    pub kode: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct GeneralResponse {
    pub kode: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValueCekToken {
    pub no_hp: String,
    pub nama_depan: String,
    pub access_token: String,
    pub refresh_token: String,
}