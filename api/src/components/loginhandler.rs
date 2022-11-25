use crate::components::err::ApiKeyError;
use crate::components::user::Role;

use serde_derive::{Deserialize, Serialize};

use chrono::*;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use shared::auth::*;

const JWT_SECRET: &[u8] = b"verleiSecret123";

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
    let header = Header::new(Algorithm::EdDSA);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).map(|v| LoginResponse {
        token: shared::auth::Token { token: v },
    })
}

use crate::routing::guards::Token;

pub fn validate_jwt(token: Option<&str>) -> Result<self::Token, ApiKeyError> {
    match token {
        Some(token) => {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::EdDSA),
            ) {
                Ok(_) => Ok(Token {
                    token: token.to_string(),
                }),
                Err(_) => Err(ApiKeyError::Invalid),
            }
        }
        None => Err(ApiKeyError::Missing),
    }
}
