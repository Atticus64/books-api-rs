use crate::models::Book;
use crate::{
    db::BooksRepository,
    dto::books::{BookDto, UpdateBookDto},
};
use graphul::{extract::Json, http::utils::Response, Context};

pub async fn get_books() -> Result<Json<Vec<Book>>, Response<String>> {
    let mut repo = BooksRepository::new();

    let books = repo.get_all_books();

    if books.is_ok() {
        Ok(Json(books.unwrap()))
    } else {
        Err(Response::builder()
            .status(400)
            .body("failed to get books".to_string())
            .unwrap())
    }
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
            return Response::builder().status(400).body(error).unwrap();
        }
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

    let (message, status) = match repo.create_book(&book) {
        Ok(_) => ("Book created", 201),
        Err(_) => ("Failed to create book", 400),
    };

    Response::builder()
        .status(status)
        .body(message.to_string())
        .unwrap()
}

fn check_payload(payload_book: &UpdateBookDto) -> bool {
    if payload_book.title.is_none()
        && payload_book.description.is_none()
        && payload_book.author.is_none()
        && payload_book.published.is_none()
    {
        false
    } else {
        true
    }
}

fn validate_payload(payload_book: UpdateBookDto, mut book: Book) -> Book {
    book.title = payload_book.title.unwrap_or(book.title);
    book.author = payload_book.author.unwrap_or(book.author);
    book.description = payload_book.description.unwrap_or(book.description);
    book.published = payload_book.published.unwrap_or(book.published);

    book
}

pub async fn update_book(c: Context) -> Response<String> {
    let mut repo = BooksRepository::new();
    let id = c.params("id").parse::<i32>().unwrap();

    let book_data = match c.payload::<UpdateBookDto>().await {
        Ok(data) => data,
        Err(e) => {
            let error = format!("Failed to parse book, error: {}", e.body_text().as_str());
            return Response::builder().status(400).body(error).unwrap();
        }
    };

    let valid = check_payload(&book_data.0);

    if !valid {
        return Response::builder()
            .status(400)
            .body("Book is not valid".to_string())
            .unwrap();
    }

    let book = repo.find_book(id);

    match book {
        Ok(_) => {}
        Err(_) => {
            return Response::builder()
                .status(404)
                .body("Book not found or not exist".to_string())
                .unwrap();
        }
    }

    let b = book.unwrap();
    let data = validate_payload(book_data.0, b);

    let book = crate::models::NewBook {
        title: data.title.as_str(),
        author: data.author.as_str(),
        description: data.description.as_str(),
        published: data.published,
    };

    match repo.update_book(id, &book) {
        Ok(_) => {}
        Err(_) => {
            return Response::builder()
                .status(400)
                .body("Failed to update book".to_string())
                .unwrap();
        }
    }

    Response::builder()
        .status(201)
        .body("Book updated".to_string())
        .unwrap()
}

pub async fn delete_book(c: Context) -> Response<String> {
    let mut repo = BooksRepository::new();
    let id = c.params("id").parse::<i32>().unwrap();

    let book = match repo.delete_book(id) {
        Ok(b) => b,
        Err(_) => {
            return Response::builder()
                .status(404)
                .body("Book not found or not exist".to_string())
                .unwrap();
        }
    };

    Response::builder()
        .status(200)
        .body(book.to_string())
        .unwrap()
}
