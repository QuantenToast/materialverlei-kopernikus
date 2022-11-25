use rocket::http::Status;
use serde::{Deserialize, Serialize};
use shared::auth::{LoginRequest, LoginResponse};

use super::loginhandler::create_jwt;
use crate::db::user::get_user;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uname: String,
    pub pwd: String,
    pub role: Role,
    pub email: String,
}

pub async fn req_login(lr: LoginRequest) -> std::result::Result<LoginResponse, Status> {
    if let Ok(v) = get_user(&lr.username).await {
        if v.pwd != lr.password {
            return Err(Status::Unauthorized);
        }
        create_jwt(&v.uname, &v.role)
            .map(|login_response| login_response)
            .map_err(|_| Status::NotAcceptable)
    } else {
        Err(Status::NotAcceptable)
    }
}

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

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Role {
        match s {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}
