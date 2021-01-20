use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello World")
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("New Server is starting up.");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/api/v0", web::get().to(home))
    }).bind(("0.0.0.0", 3000))?
        .run()
        .await
}
