use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing_futures::Instrument;

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
    let tracing_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_name = %form.name,
        subscriber_email = %form.email
    );
    let _request_span_guard = tracing_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber in the database");
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
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id: {} New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id: {} Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
