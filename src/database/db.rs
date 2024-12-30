use std::{env, sync::{Arc, Mutex}};

use dotenvy::dotenv;
use duckdb::{Result, Connection};
pub fn initialize_db() -> Result<Arc<Mutex<Connection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Connection::open(database_url)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            no_hp VARCHAR PRIMARY KEY,
            nama_depan VARCHAR NOT NULL,
            nama_belakang VARCHAR NOT NULL,
            password TEXT NOT NULL,
            otp VARCHAR,
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS token_user (
            no_hp VARCHAR PRIMARY KEY,
            nama_depan VARCHAR NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT NOT NULL,
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS version (
            id VARCHAR PRIMARY KEY, 
            version_major VARCHAR, 
            version_minor VARCHAR, 
            version_patch VARCHAR, 
            platform VARCHAR, 
            latest BOOLEAN, 
            allowed BOOLEAN, 
            created_at TIMESTAMP, 
            created_by VARCHAR, 
            updated_at TIMESTAMP, 
            updated_by VARCHAR,
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (
            id VARCHAR PRIMARY KEY,
            title VARCHAR, 
            description TEXT, 
            created_at TIMESTAMP, 
            created_by VARCHAR, 
            updated_at TIMESTAMP, 
            updated_by VARCHAR,
        )",
        [],
    )?;
    Ok(Arc::new(Mutex::new(conn)))
}