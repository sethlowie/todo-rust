extern crate actix_web;

use actix_web::error::ErrorForbidden;
use actix_web::http::header;
use actix_web::middleware::{Middleware, Started};
use actix_web::{http, server, App, HttpRequest, Result};
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

fn create(_req: &HttpRequest) -> &'static str {
    run("INSERT INTO waffles (name) VALUES ('syrup')");
    "created"
}

fn fetch(_req: &HttpRequest) -> String {
    let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("Could not connect");

    let mut res = "".to_owned();

    for row in &conn.query("SELECT name FROM waffles", &[]).unwrap() {
        let name: String = row.get(0);
        res = format!("{}, {}", res, name);
    }

    res
}

fn run(query: &str) {
    let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("Could not connect");

    conn.execute(query, &[]).expect("Could not create table");
}

fn main() {
    // run("CREATE TABLE waffles (
    //         name VARCHAR NOT NULL
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
                .resource("/create", |r| r.f(create))
                .resource("/fetch", |r| r.f(fetch))
                .finish(),
        ]
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}
