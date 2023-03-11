use crate::schema::MyTable as file_attachment;
use diesel;
use diesel::{Insertable, Queryable};
use serde::*;

#[derive(Queryable)]
pub struct Attachment {
    pub ID: i32,
    pub ContentType: String,
    pub AttachmentName: String,
    pub AttachmentData: Vec<u8>,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name=file_attachment)]
pub struct NewAttachment {
    pub ContentType: String,
    pub AttachmentName: String,
    pub AttachmentData: Vec<u8>,
}
