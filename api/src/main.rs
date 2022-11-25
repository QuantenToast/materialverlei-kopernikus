#![feature(decl_macro, proc_macro_hygiene)]

mod components;
mod db;
mod routing;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use routing::routs;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/", routes![routs::get_page])
        .mount("/auth/", routes![routs::authenticate])
        .mount("/", routes![routs::index, routs::get_file])
}
