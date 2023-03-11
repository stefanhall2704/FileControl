-- Your SQL goes here
CREATE TABLE MyTable (
  ID INTEGER PRIMARY KEY NOT NULL,
  ContentType VARCHAR(100) NOT NULL,
  AttachmentName VARCHAR(100) NOT NULL,
  AttachmentData BLOB NOT NULL
);
