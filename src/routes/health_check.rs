use actix_web::HttpResponse;
pub async fn healt_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
