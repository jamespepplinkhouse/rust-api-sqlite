use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Deserialize)]
pub struct UserId {
    pub id: i32,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

pub async fn get_user(pool: web::Data<SqlitePool>, user_id: web::Path<UserId>) -> impl Responder {
    let id = user_id.id;
    let result = sqlx::query("SELECT id, name, age FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await;

    match result {
        Ok(row) => {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                age: row.get("age"),
            };
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}
