#![feature(decl_macro, proc_macro_hygiene)]

mod components;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use components::handlers;
use rocket::fs::FileServer;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/api/", routes![handlers::get_page])
}
