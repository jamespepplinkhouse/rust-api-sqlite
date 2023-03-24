use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn greet_user(user: web::Path<User>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello, {}! You are {} years old.",
        user.name, user.age
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/user/{name}/{age}", web::get().to(greet_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
