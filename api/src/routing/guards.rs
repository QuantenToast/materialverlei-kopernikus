pub struct Token {
    pub token: String,
}

use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::Request;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");

        match validate_jwt(token) {
            Ok(v) => Success(v),
            Err(e) => Failure((Status::Unauthorized, e)),
        }
    }
}

use rocket::fs::NamedFile;
use rocket::request::{FromRequest, Outcome};
use std::path::PathBuf;

use crate::components::loginhandler::validate_jwt;

use crate::components::err::ApiKeyError;

pub struct AuthRes {
    pub res: Result<NamedFile, Status>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthRes {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::outcome::Outcome::*;

        match NamedFile::open(
            PathBuf::from("static").join(request.param::<PathBuf>(0).unwrap().unwrap()),
        )
        .await
        {
            Ok(v) => Success(Self { res: Ok(v) }),
            Err(_) => match request.guard::<Token>().await {
                Success(_) => Success(Self {
                    res: crate::routing::helpers::get_index().await,
                }),
                Failure(e) => Failure(e),
                _ => Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            },
        }
    }
}
