use std::env;
use std::error::Error;

use rocket::http::Status;

use super::db::get_page_db;

#[get("/<num>")]
pub async fn get_page(num: u32) -> Result<String, Status> {
    get_page_db(num)
        .await
        .map(|v| v)
        .map_err(|e| error_status(e))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(e: Box<dyn Error>) -> Status {
    match e {
        _ => {
            eprintln!("{:?}", e);
            Status::BadRequest
        }
    }
}
