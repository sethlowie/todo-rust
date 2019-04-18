#[macro_use]
extern crate serde_derive;
extern crate actix_web;

use actix_web::error::ErrorForbidden;
use actix_web::http::header;
use actix_web::middleware::{Middleware, Started};
use actix_web::{http, server, App, HttpRequest, Json, Result};
use postgres::{Connection, TlsMode};

struct Auth;

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        match req.headers().get(header::AUTHORIZATION) {
            Some(_auth) => Ok(Started::Done),
            None => Err(ErrorForbidden("")),
        }
    }
}

fn handler(_req: &HttpRequest) -> &'static str {
    "hello api"
}

fn login(_req: &HttpRequest) -> &'static str {
    "login"
}

fn create(todo: Json<Todo>) -> Result<String> {
    run(&format!(
        "INSERT INTO todos (title, description) VALUES ('{}', '{}')",
        todo.title, todo.description
    )
    .to_string());
    Ok("created".to_string())
}

fn fetch(_req: &HttpRequest) -> Json<Vec<Todo>> {
    let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("Could not connect");

    let mut res: Vec<Todo> = vec![];

    for row in &conn
        .query("SELECT title, description FROM todos", &[])
        .unwrap()
    {
        let todo = Todo {
            title: row.get(0),
            description: row.get(1),
        };
        res.push(todo);
    }

    Json(res)
}

fn run(query: &str) {
    let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("Could not connect");

    conn.execute(query, &[]).expect("Could not create table");
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    description: String,
}

fn main() {
    // run("CREATE TABLE todos (
    //         title VARCHAR NOT NULL,
    //         description VARCHAR NOT NULL
    //     )");

    server::new(|| {
        vec![
            App::new()
                .prefix("/auth")
                .resource("/login", |r| r.f(login))
                .resource("/test", |r| r.method(http::Method::POST).f(login))
                .finish(),
            App::new()
                .middleware(Auth)
                .prefix("/api")
                .resource("", |r| r.f(handler))
                .resource("/", |r| r.f(handler))
                .resource("/create", |r| r.method(http::Method::POST).with(create))
                .resource("/fetch", |r| r.f(fetch))
                .finish(),
        ]
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}
