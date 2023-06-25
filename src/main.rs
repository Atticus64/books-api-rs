use graphul::{http::Methods, Graphul};
use controllers::books::{create_book, get_books};

mod db;
mod models;
mod schema;
mod controllers;

#[tokio::main]
async fn main() {
  let mut app = Graphul::new();

  app.get("/books", get_books);

  app.post("/create", create_book);
  // routes out of the scope of the middleware
  app.get("/login", || async {
      "Login page"
  });

  app.run("127.0.0.1:8000").await;
}


