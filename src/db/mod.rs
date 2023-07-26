use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{Book, BookData, NewBook};

pub struct BooksRepository {
    conn: PgConnection,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl BooksRepository {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(),
        }
    }

    pub fn find_book(&mut self, idt: i32) -> Result<Book, diesel::result::Error> {
        use crate::schema::books::dsl::*;

        let b = books
            .filter(id.eq(idt))
            .load::<Book>(&mut self.conn)
            .expect("Error loading book");

        if b.is_empty() {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(b.get(0).unwrap().clone())
    }

    pub fn get_all_books(&mut self) -> Result<Vec<Book>, diesel::result::Error> {
        use crate::schema::books::dsl::*;

        let result = books.filter(deleted.eq(false)).load::<Book>(&mut self.conn);

        match result {
            Ok(collection) => Ok(collection),
            Err(e) => Err(e),
        }
    }

    pub fn create_book(&mut self, new_book: &NewBook) -> Result<Book, diesel::result::Error> {
        use crate::schema::books::dsl::*;

        let res = diesel::insert_into(books)
            .values(new_book)
            .returning(Book::as_returning())
            .get_result(&mut self.conn);

        match res {
            Ok(b) => Ok(b),
            Err(e) => Err(e),
        }
    }

    pub fn update_book(
        &mut self,
        identifier: i32,
        new_book: &NewBook,
    ) -> Result<Book, diesel::result::Error> {
        use crate::schema::books::dsl::*;

        let data = BookData {
            title: new_book.title.to_string(),
            description: new_book.description.to_string(),
            author: new_book.author.to_string(),
            published: new_book.published,
        };

        match diesel::update(books)
            .set(data)
            .filter(id.eq(identifier))
            .returning(Book::as_returning())
            .get_result(&mut self.conn)
        {
            Ok(book) => Ok(book),
            Err(e) => Err(e),
        }
    }

    pub fn delete_book(&mut self, identifier: i32) -> Result<Book, diesel::result::Error> {
        use crate::schema::books::dsl::*;

        let book = match self.find_book(identifier) {
            Ok(b) => b,
            Err(e) => {
                return Err(e);
            }
        };

        let result = diesel::update(books)
            .set(deleted.eq(true))
            .filter(id.eq(identifier))
            .execute(&mut self.conn);

        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(book)
    }
}
