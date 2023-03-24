use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use sqlx::{Row, SqlitePool};
use std::env;

#[derive(Deserialize)]
struct UserId {
    id: i32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_user(pool: web::Data<SqlitePool>, user_id: web::Path<UserId>) -> impl Responder {
    let id = user_id.id;
    let result = sqlx::query("SELECT id, name, age FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await;

    match result {
        Ok(row) => {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            let age: i32 = row.get("age");
            HttpResponse::Ok().body(format!("User: ID={} Name={} Age={}", id, name, age))
        }
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:users.db".to_string());
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create SQLite connection pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
