use rocket::serde::json::Json;
use rocket::{http::Status, response::Redirect};

use super::{
    db::get_page_db,
    loginhandler::{req_login, LoginRequest, LoginResponse, Token},
};
use rocket::fs::NamedFile;
use std::path::PathBuf;

#[get("/<num>", rank = 0)]
pub async fn get_page(num: u32) -> Result<String, Status> {
    get_page_db(num)
        .await
        .map(|v| v)
        .map_err(|e| error_status(e))
}

#[get("/", rank = 1)]
pub async fn index() -> Result<NamedFile, Status> {
    get_index().await
}

#[post("/", format = "application/json", data = "<lr>")]
pub async fn authenticate(lr: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    req_login(lr.into_inner()).await.map(|v| Json(v))
}

#[get("/<path..>", rank = 3)]
pub async fn secure(path: PathBuf, _auth: Token) -> Result<NamedFile, Status> {
    let path = PathBuf::from("static").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/<object>", rank = 2)]
pub async fn check_if_file(object: PathBuf) -> Redirect {
    match NamedFile::open(PathBuf::from("static").join(&object)).await {
        Ok(_) => Redirect::to(uri!(get_file(&object))),
        Err(_) => Redirect::to(uri!(secure(object))),
    }
}

#[get("/<object>", rank = 99)]
pub async fn get_file(object: PathBuf) -> Result<NamedFile, Status> {
    NamedFile::open(PathBuf::from("static").join(object))
        .await
        .map_err(|_| rocket::http::Status::NotFound)
}

pub async fn get_index() -> Result<NamedFile, Status> {
    NamedFile::open("static/index.html")
        .await
        .map_err(|_| rocket::http::Status::NotFound)
}

fn error_status(e: anyhow::Error) -> Status {
    match e {
        _ => Status::BadRequest,
    }
}
