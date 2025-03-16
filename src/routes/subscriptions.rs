use actix_web::{web, HttpResponse};
use sqlx::{query, PgConnection};
use syn::Data;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
// Let's start simple: we always return a 200 OK
pub async fn subscribe(form: web::Form<FormData>, _connection : web::Data<PgConnection>,) -> HttpResponse {

    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    ).execute(_connection.get_ref()).await;

    HttpResponse::Ok().finish()
}