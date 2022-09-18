use crate::error::Error;

use super::req::{get_token, request_post, set_token};
use shared::types::auth::*;
use yew_router::history::History;
use yew_router::hooks::use_history;

pub async fn login(lr: LoginRequest) -> Result<LoginResponse, Error> {
    match get_token() {
        Some(t) => Ok(LoginResponse { token: t }),
        None => {
            let res =
                request_post::<LoginRequest, LoginResponse>(String::from("login"), lr).await?;
            set_token(Some(res.token.clone()));
            Ok(res)
        }
    }
}

pub fn logout() {
    set_token(None);
    use_history().unwrap().push(crate::Route::Home);
}
