use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("FFF {name}!")
}

#[get("/foo/{i}")]
async fn foo(i: web::Path<String>) -> impl Responder {
    format!("AAA {i}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(foo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
