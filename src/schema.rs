// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        description -> Text,
        published -> Bool,
        deleted -> Bool,
    }
}
