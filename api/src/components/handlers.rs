use rocket::http::Status;

use super::{db::get_page_db, err::ApiKeyError};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

pub async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<path..>", rank = 3)]
pub async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("static").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/", rank = 1)]
pub async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[get("/<num>", rank = 2)]
pub async fn get_page(num: u32) -> Result<String, (Status, &'static str)> {
    get_page_db(num)
        .await
        .map(|v| v)
        .map_err(|e| error_status(e))
}

fn error_status(e: anyhow::Error) -> (Status, &'static str) {
    match e {
        _ => {
            eprintln!("{:?}", e);
            (Status::BadRequest, "BadRequest")
        }
    }
}
