use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct BookDto {
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
}
