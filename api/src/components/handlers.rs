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

fn error_status(e: Box<dyn Error>) -> Status {
    match e {
        _ => {
            eprintln!("{:?}", e);
            Status::BadRequest
        }
    }
}
