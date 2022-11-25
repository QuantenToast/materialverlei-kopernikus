use crate::db::get_conn;
use bson::Document;

use crate::components::material::{Material, MaterialRes};
use futures::stream::StreamExt;

use anyhow::Result;

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
