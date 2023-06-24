use db::establish_connection;
use models::Book;
use serde_json::json;
use graphul::{http::Methods, Graphul, Context, extract::Json};
use serde::{Deserialize, Serialize};

mod db;
mod models;
mod schema;

#[derive(Serialize, Deserialize)]
struct BookDto {
    pub title: String,
    pub description: String,
    pub author: String,
}

impl Default for BookDto {
    fn default() -> Self {
        Self {
            title: String::from(""),
            description: String::from(""),
            author: String::from(""),
        }
    }

}

async fn get_books()  -> Json<Vec<Book>> {

    let books = db::get_all_books();

    Json(books)
}

#[tokio::main]
async fn main() {
  let mut app = Graphul::new();

  app.get("/books", get_books);

  app.post("/create", |c: Context| async move {


      let book_data = match c.payload::<BookDto>().await {
          Ok(data) => data,
          Err(_) => {
              return "Failed to parse book, check fields title, author, description";
          },
      };

      
      let book = models::NewBook {
          title: book_data.title.as_str(),
          author: book_data.author.as_str(),
          description: book_data.description.as_str(),
      };

      db::create_book(&mut establish_connection(), &book);

      "Book created"
  });
  // routes out of the scope of the middleware
  app.get("/login", || async {
      "Login page"
  });

  app.run("127.0.0.1:8000").await;
}


