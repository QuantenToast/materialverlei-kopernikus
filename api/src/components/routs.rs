use rocket::{self, fs::FileServer};

use crate::components::handlers;

pub async fn create_routes() {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/api/", routes![handlers::get_page])
        .launch();
}
