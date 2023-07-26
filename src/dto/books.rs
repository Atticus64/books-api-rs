use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct BookDto {
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpdateBookDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
}