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