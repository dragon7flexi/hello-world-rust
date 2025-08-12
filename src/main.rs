use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "msg": "hello world" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
