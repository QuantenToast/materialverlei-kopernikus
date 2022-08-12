extern crate bson;
extern crate chrono;
extern crate futures;
extern crate mongodb;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use bson::Document;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;

use super::material::{Material, MaterialRes};
use futures::stream::StreamExt;

pub async fn get_page_db(num: u32) -> Result<String, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

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
                // Use serde to deserialize into the MovieSummary struct:
                let doc: Material = bson::from_document(result?)?;

                mats.push(MaterialRes {
                    id: doc.id.unwrap().to_hex(),
                    name: doc.name,
                    description: doc.description,
                    pic: doc.pic,
                    num_available: doc.num_available
                });
            }
        }
        Err(e) => eprintln!("{:?}", e),
    };

    let res = serde_json::to_string(&mats)?;

    Ok(res)
}
