#![feature(decl_macro, proc_macro_hygiene)]

mod components;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use components::handlers;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api/", routes![handlers::get_page])
        .mount("/auth/", routes![handlers::authenticate])
        .mount("/secure/", routes![handlers::secure])
        .mount("/", routes![handlers::index, handlers::static_files])
}
