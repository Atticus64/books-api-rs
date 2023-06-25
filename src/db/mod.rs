use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{Book, NewBook};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_book(conn: &mut PgConnection, new_book: &NewBook) -> Book {
    use crate::schema::books;

    diesel::insert_into(books::table)
        .values(new_book)
        .returning(Book::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}


pub fn get_all_books() -> Vec<Book> {
    use crate::schema::books::dsl::*;

    let mut conn = establish_connection();

    books
        .filter(deleted.eq(false))
        .load::<Book>(&mut conn)
        .expect("Error loading posts")
}
