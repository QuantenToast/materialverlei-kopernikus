use super::{
    db::get_page_db,
    loginhandler::{req_login, AuthRes},
};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::path::PathBuf;

use shared::auth::{LoginRequest, LoginResponse};

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

#[get("/<path..>", rank = 2)]
pub async fn get_file(path: PathBuf, pauth: AuthRes) -> Result<NamedFile, Status> {
    Ok(pauth.res?)
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
