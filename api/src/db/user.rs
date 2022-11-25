use crate::db::get_conn;
use anyhow::Result;
use bson::{doc, Document};

use crate::components::{err::ApiKeyError, user::User};

pub async fn get_user(usr: &String) -> Result<User> {
    let conn = get_conn().await?;

    let col: mongodb::Collection<Document> = conn.database("material-verlei").collection("users");
    match col
        .find_one(
            doc! {
                "username": usr
            },
            None,
        )
        .await?
    {
        Some(c) => {
            let doc: User = bson::from_document(c)?;

            Ok(User {
                uname: doc.uname,
                pwd: doc.pwd,
                role: doc.role,
                email: doc.email,
            })
        }
        None => Err(ApiKeyError::Invalid.into()),
    }
}
