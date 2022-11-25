use crate::components::err::ApiKeyError;
use crate::components::user::Role;

use serde_derive::{Deserialize, Serialize};

use chrono::*;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

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
            use hex;
            use sha3::{Digest, Sha3_256};
            let mut hasher = Sha3_256::new();
            hasher.update(v.pwd.as_bytes());
            let pwd = hex::encode(hasher.finalize());
            if pwd != lr.password {
                return Err(Status::Unauthorized);
            }
            create_jwt(&v.uname, &v.role)
                .map(|login_response| login_response)
                .map_err(|_| Status::NotAcceptable)
        }
        Err(_) => Err(Status::NotAcceptable),
    }
}
