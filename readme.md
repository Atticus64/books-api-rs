# Api Books 

Example of Backend api rest in Rust 🦀

### Technologies 

* Graphul
* Postgresql
* Diesel.rs

## Setup

1. *Install Diesel.rs cli*

```bash
cargo install diesel_cli
```

2. Clone repository 

```bash
git clone https://github.com/atticus64/books-api-rs
```

3. Start postgresql database (example with docker)

```yml
version: "3.3"

services:
  db1:
    container_name: rusty_postgre
    image: postgres:13.4-alpine
    ports:
      - 5432:5432
    environment:
      POSTGRES_PASSWORD: postgres
      POSTGRES_USER: postgres
      POSTGRES_DB: books
```

* Execute docker-compose

```bash
docker compose up
```

4. Run migrations with diesel

```bash 
diesel migration run
```

5. Run server 

```
cargo run
```

### Endpoints

* List books

```bash
curl localhost:8000/books
```

* Create book

```bash
curl localhost:8000/create -H 'Content-type: application/json' \
```

* Book in body Post Create and Update

```rs
struct BookData {
    pub title: String,
    pub description: String,
    pub author: String,
    pub published: bool,
}
```


* Update Book

```bash
curl -X PUT localhost:8000/books/{id}
```


* Delete Book

```bash
curl -X DELETE localhost:8000/books/{id}
```



