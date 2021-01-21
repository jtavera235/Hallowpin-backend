use actix_web::{get, web, App, HttpResponse, HttpServer, HttpRequest, Responder};
use std::env;

fn set_environment_vars() {
    let port = "DEFAULT_PORT";
    let api_version = "API_VERSION";
    env::set_var(port, String::from("3000"));
    env::set_var(api_version, "v0");
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
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
            .route("/health_check", web::get().to(health_check))
    }).bind(("0.0.0.0", port))?
        .run()
        .await
}
