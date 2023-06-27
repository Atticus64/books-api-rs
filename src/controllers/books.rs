use crate::db::{BooksRepository};
use crate::models::Book;
use graphul::{Context, extract::Json, http::utils::Response};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
struct BookDto {
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
}

pub async fn get_books()  -> Json<Vec<Book>> {
    let mut repo = BooksRepository::new();

    let books = repo.get_all_books();

    Json(books)
}

fn book_validation(book: &BookDto) -> Result<(), String> {
    if book.title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    if book.description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }
    if book.author.is_empty() {
        return Err("Author cannot be empty".to_string());
    }

    Ok(())
}


pub async fn create_book(c: Context) -> Response<String> {
    let mut repo = BooksRepository::new();
    let book_data = match c.payload::<BookDto>().await {
        Ok(data) => data,
        Err(e) => {

            let error = format!("Failed to create book, error: {}", e.body_text().as_str());
            return Response::builder()
                .status(400)
                .body(error)
                .unwrap()
        },
    };


    match book_validation(&book_data) {
        Ok(_) => {}
        Err(e) => {
            return Response::builder().status(400).body(e).unwrap();
        }
    };

    let book = crate::models::NewBook {
        title: book_data.title.as_str(),
        author: book_data.author.as_str(),
        description: book_data.description.as_str(),
        published: book_data.published,
    };

    repo.create_book(&book);

    Response::builder()
        .status(201)
        .body("Book created".to_string())
        .unwrap()
}


pub async fn update_book(c: Context) -> Response<String> {
    let mut repo = BooksRepository::new();
    let id = c.params("id").parse::<i32>().unwrap();
 
    let book_data = match c.payload::<BookDto>().await {
        Ok(data) => data,
        Err(e) => {

            let error = format!("Failed to create book, error: {}", e.body_text().as_str());
            return Response::builder()
                .status(400)
                .body(error)
                .unwrap()
        },
    };


    match book_validation(&book_data) {
        Ok(_) => {}
        Err(e) => {
            return Response::builder().status(400).body(e).unwrap();
        }
    };

    let book = crate::models::NewBook {
        title: book_data.title.as_str(),
        author: book_data.author.as_str(),
        description: book_data.description.as_str(),
        published: book_data.published,
    };

    repo.update_book(id, &book);

    Response::builder()
        .status(201)
        .body("Book updated".to_string())
        .unwrap()
}

pub async fn delete_book(c: Context) -> Response<String> {
    let mut repo = BooksRepository::new();
    let id = c.params("id").parse::<i32>().unwrap();

    repo.delete_book(id);

    Response::builder()
        .status(201)
        .body("Book deleted".to_string())
        .unwrap()

}
