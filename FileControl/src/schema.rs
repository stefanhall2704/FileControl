// @generated automatically by Diesel CLI.

diesel::table! {
    MyTable (ID) {
        ID -> Integer,
        ContentType -> Text,
        AttachmentName -> Text,
        AttachmentData -> Binary,
    }
}
