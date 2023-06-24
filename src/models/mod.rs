use diesel::prelude::*;
use crate::schema::books;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub author: &'a str,
}

