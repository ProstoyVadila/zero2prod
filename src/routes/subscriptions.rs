use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

pub async fn subcribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
