use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subcribe(
    form: web::Form<FormData>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {

    let request_id = Uuid::new_v4();
    log::info!(
        "request_id: {} Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.name,
        form.email
        );
    log::info!("Saing new subscriber details in the database");
    match sqlx::query!(
            r#"
            insert into subscriptions(id, email, name, subscribed_at)
            values ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
        )
        .execute(pg_pool.get_ref())
        .await
    {
        Ok(_) => {
            log::info!(
                "request_id: {} New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("request_id: {} Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
