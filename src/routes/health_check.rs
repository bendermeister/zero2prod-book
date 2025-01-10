use actix_web::HttpResponse;

#[tracing::instrument("Health Check")]
pub async fn healt_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
