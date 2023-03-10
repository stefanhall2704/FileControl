#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
use rocket::routes;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use self::api::*;
pub mod api;

fn main() {
    rocket::ignite()
        .mount("/api", routes![download_file, upload_file_api])
        .launch();
}
