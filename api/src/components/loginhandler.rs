use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use serde_derive::{Deserialize, Serialize};

use chrono::*;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use super::db::get_user;
use super::err::ApiKeyError;

use shared::auth::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uname: String,
    pub pwd: String,
    pub role: Role,
}

pub async fn req_login(lr: LoginRequest) -> std::result::Result<LoginResponse, Status> {
    match get_user(&lr.username).await {
        Ok(v) => {
            if v.pwd != lr.password {
                return Err(Status::Unauthorized);
            }
            create_jwt(&v.uname, &v.role)
                .map(|login_response| login_response)
                .map_err(|_| Status::NotAcceptable)
        }
        Err(_) => Err(Status::NotAcceptable),
    }
}

const JWT_SECRET: &[u8] = b"verleiSecret123";

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn to_string(&self) -> String {
        match &self {
            Self::Admin => String::from("Admin"),
            Self::User => String::from("User"),
        }
    }

    pub fn from_str(s: &str) -> Role {
        match s {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn create_jwt(usrname: &str, role: &Role) -> Result<LoginResponse, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(2))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: usrname.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).map(|v| LoginResponse {
        token: shared::auth::Token { token: v },
    })
}

pub struct Token {
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        match token {
            Some(token) => {
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(JWT_SECRET),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_) => Outcome::Success(Token {
                        token: token.to_string(),
                    }),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
        }
    }
}
