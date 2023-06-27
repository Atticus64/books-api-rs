use graphul::{http::Methods, Graphul};
use crate::controllers::books::{create_book, get_books, update_book, delete_book};

pub fn book_router() -> Graphul {
    let mut book_router = Graphul::new();

    book_router.get("/books", get_books);
    book_router.post("/create", create_book);
    book_router.put("/books/:id", update_book);
    book_router.delete("/books/:id", delete_book);

    book_router
}

