use crate::schema::books;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
    pub deleted: bool,
}

impl Book {
    pub fn to_string(&self) -> String {
        format!(
            "id: {}, title: {}, description: {}, author: {}, published: {}, deleted: {}",
            self.id, self.title, self.description, self.author, self.published, self.deleted
        )
    }
}

#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = books)]
pub struct BookData {
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub author: &'a str,
    pub published: bool,
}
