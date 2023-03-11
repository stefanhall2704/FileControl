#![allow(non_snake_case)]
use rocket::{get, post};
use rocket::{http::ContentType, Data};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::{to_string, Value};
use std::result::Result;
use FileControl::*;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[post("/attachment/<filename>", data = "<data>")]
pub fn upload_file_api(
    content_type: &ContentType,
    data: Data,
    filename: String,
) -> Result<Vec<u8>, std::io::Error> {
    let connection = &mut establish_connection();

    let mut buffer = Vec::new();
    data.stream_to(&mut buffer)?;
    let content_type: String = content_type.to_string();

    // let filename: String = "stefans_cert.pdf".to_string();
    let file = buffer.clone();
    upload_db_file(connection, file, filename, content_type);

    Ok(buffer)
}

#[get("/download/<id>")]
pub fn download_file(id: i32) -> Result<Result<std::fs::File, std::io::Error>, ()> {
    let file_download = download_db_file(id);
    Ok(file_download)
}
