use actix_web::HttpResponse;
use askama::Template;
mod template;
use template::IndexTemplate;
pub mod user_handlers;

pub use user_handlers::{get_user, UserId};

pub async fn index() -> actix_web::Result<HttpResponse> {
    let tmpl = IndexTemplate;
    tmpl.render()
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering failed"))
}
