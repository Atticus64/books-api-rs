use crate::db::establish_connection;
use crate::models::Book;
use graphul::{Context, extract::Json};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
struct BookDto {
    pub title: String,
    pub description: String,
    pub author: String,
}


pub async fn get_books()  -> Json<Vec<Book>> {

    let books = crate::db::get_all_books();

    Json(books)
}


pub async fn create_book(c: Context) -> &'static str {
    let book_data = match c.payload::<BookDto>().await {
        Ok(data) => data,
        Err(_) => {
            return "Failed to parse book, check fields title, author, description";
        },
    };


    let book = crate::models::NewBook {
        title: book_data.title.as_str(),
        author: book_data.author.as_str(),
        description: book_data.description.as_str(),
    };

    crate::db::create_book(&mut establish_connection(), &book);

    "Book created"

}

