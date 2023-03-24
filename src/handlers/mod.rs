use actix_web::{HttpResponse, Responder};

pub mod user_handlers;

pub use user_handlers::{get_user, UserId};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
