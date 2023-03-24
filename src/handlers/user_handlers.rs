use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Row, SqlitePool};

#[derive(Deserialize)]
pub struct UserId {
    pub id: i32,
}

pub async fn get_user(pool: web::Data<SqlitePool>, user_id: web::Path<UserId>) -> impl Responder {
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
