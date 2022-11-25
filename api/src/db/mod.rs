pub mod material;
pub mod user;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;

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
