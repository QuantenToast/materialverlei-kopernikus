use bson::{doc, Document};

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;

use super::{
    err::ApiKeyError,
    material::{Material, MaterialRes},
};
use futures::stream::StreamExt;

use super::loginhandler::User;
use anyhow::Result;

async fn get_conn() -> mongodb::error::Result<Client> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    Client::with_options(options)
}

pub async fn get_page_db(num: u32) -> Result<String> {
    let client = get_conn().await?;

    let col: mongodb::Collection<Document> =
        client.database("material-verlei").collection("materialien");

    let pipeline = vec![
        bson::doc! {
            "$skip": num * 50
        },
        bson::doc! {
            "$limit": 50
        },
        // bson::doc! {
        //     "$sort": {"$natural": 1}
        // },
    ];

    let mut mats: Vec<MaterialRes> = Vec::new();

    match col.aggregate(pipeline, None).await {
        Ok(mut c) => {
            while let Some(result) = c.next().await {
                // Use serde to deserialize into the Material struct:
                let mut doc: Material = bson::from_document(result?)?;

                if let Some(date) = doc.borrow_time {
                    if date.1 < chrono::Local::today().naive_local() {
                        doc.borrow_time = None;
                    }
                };

                mats.push(MaterialRes {
                    id: doc.id.unwrap().to_hex(),
                    name: doc.name,
                    description: doc.description,
                    pic: doc.pic,
                    num_available: doc.num_available,
                    borrower: doc.borrower,
                    borrow_time: doc.borrow_time,
                    damage: doc.damage,
                });
            }
        }
        Err(e) => eprintln!("{:?}", e),
    };

    let res = serde_json::to_string(&mats)?;

    Ok(res)
}

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
