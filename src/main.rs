use graphul::Graphul;
use routes::books::book_router;

mod db;
mod models;
mod schema;
mod controllers;
mod routes;

#[tokio::main]
async fn main() {
  let mut app = Graphul::new();

  app.add_router(book_router());

  app.run("127.0.0.1:8000").await;
}


