use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

fn set_environment_vars() {
    let key = "DEFAULT_PORT";
    env::set_var(key, String::from("3000"));
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello World")
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    set_environment_vars();

    let default_port = match env::var("DEFAULT_PORT") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };
    let port = env::var("PORT")
        .unwrap_or_else(|_| default_port)
        .parse()
        .expect("PORT must be a number");

    println!("Server starting in port {:?}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/api/v0", web::get().to(home))
    }).bind(("0.0.0.0", port))?
        .run()
        .await
}
