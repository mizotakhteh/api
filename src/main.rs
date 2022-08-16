use actix_web::{App, HttpServer, web, http::header::ContentType, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("Mizotakhteh API") }))
        )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
