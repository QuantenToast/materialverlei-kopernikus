//use std::fmt::Display;
//
//use crate::rocket::request;
//use crate::rocket::request::*;
//use rocket::http::Status;
//
//use crate::chrono::*;
//use crate::schema::users;
//use jsonwebtoken::{
//    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
//};
//
//#[derive(Debug, Serialize, Deserialize, Queryable)]
//pub struct LoginRequest {
//    pub username: String,
//    pub password: String,
//}
//
//#[derive(Serialize, Debug)]
//pub struct LoginResponse {
//    pub token: String,
//}
//
//const JWT_SECRET: &[u8] = b"secret";
//
//#[derive(Clone, PartialEq)]
//pub enum Role {
//    User,
//    Admin,
//}
//
//impl Role {
//    pub fn to_string(&self) -> String {
//        match &self {
//            Self::Admin => String::from("Admin"),
//            Self::User => String::from("User"),
//        }
//    }
//
//    pub fn from_str(s: &str) -> Role {
//        match s {
//            "Admin" => Role::Admin,
//            _ => Role::User,
//        }
//    }
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Claims {
//    sub: String,
//    role: String,
//    exp: usize,
//}
//
//pub fn create_jwt(usrname: &str, role: &Role) -> Result<LoginResponse, Error> {
//    let expiration = Utc::now()
//        .checked_add_signed(chrono::Duration::weeks(2))
//        .expect("valid timestamp")
//        .timestamp();
//
//    let claims = Claims {
//        sub: usrname.to_owned(),
//        role: role.to_string(),
//        exp: expiration as usize,
//    };
//    let header = Header::new(Algorithm::HS256);
//    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
//        .map(|v| LoginResponse { token: v })
//}
//
//pub struct Token {
//    token: String,
//}
//
//impl<'a, 'r> FromRequest<'a, 'r> for Token {
//    type Error = ();
//
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//        let token = request.headers().get_one("token");
//        match token {
//            Some(token) => {
//                match decode::<Claims>(
//                    &token,
//                    &DecodingKey::from_secret(JWT_SECRET),
//                    &Validation::new(Algorithm::HS256),
//                ) {
//                    Ok(_) => Outcome::Success(Token {
//                        token: token.to_string(),
//                    }),
//                    Err(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
//                }
//            }
//            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
//        }
//    }
//}
