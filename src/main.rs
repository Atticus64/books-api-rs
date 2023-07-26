use db::test_connection;
use graphul::Graphul;
use routes::books::book_router;

mod controllers;
mod db;
mod dto;
mod models;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();
    test_connection();
    app.add_router(book_router());


    app.run("127.0.0.1:8000").await;
}
