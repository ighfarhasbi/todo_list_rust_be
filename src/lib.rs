use database::db::initialize_db;

mod routes;
mod database;
mod helper;
mod models;
mod jwt;
mod middleware;

pub async fn run() {
    let conn = initialize_db().unwrap();
    let app = routes::router(conn);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}