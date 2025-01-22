use std::io::Error;

use actix_web::{get, main, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

#[main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 3001))?
        .run()
        .await
}
