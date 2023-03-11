use diesel::connection::Connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use std::io;
use std::io::Write;

use self::models::Attachment as attachment_model;
use self::models::NewAttachment;
use self::schema::MyTable as attachment_schema;

pub mod models;
pub mod schema;

#[allow(non_snake_case)]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn upload_db_file(
    conn: &mut SqliteConnection,
    file: Vec<u8>,
    file_name: String,
    content_type: String,
) {
    let new_file = NewAttachment {
        AttachmentData: file,
        AttachmentName: file_name,
        ContentType: content_type,
    };
    diesel::insert_into(attachment_schema::table)
        .values(&new_file)
        .execute(conn)
        .expect("Error inserting data into database");
}

pub fn download_db_file(id: i32) -> io::Result<std::fs::File> {
    let connection = &mut establish_connection();
    let file_data = attachment_schema::table
        .filter(attachment_schema::ID.eq(id))
        .first::<attachment_model>(connection)
        .unwrap();
    let data = file_data.AttachmentData;
    let file_name = file_data.AttachmentName;
    let download_dir = match dirs_2::download_dir() {
        Some(path) => path,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Download directory not found",
            ))
        }
    };
    let file_path = download_dir.join(&file_name);
    let mut file = std::fs::File::create(&file_path)?;
    let data_ref: &[u8] = &data;
    file.write_all(data_ref)?;
    Ok(file)
}
