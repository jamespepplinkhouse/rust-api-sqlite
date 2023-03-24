use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use std::env;

mod handlers;
use handlers::index;
use handlers::user_handlers::get_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:users.db".to_string());
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create SQLite connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
