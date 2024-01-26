use std::net::Ipv4Addr;
use std::io::Result;
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body("We are the Reddy's Vault.")
}

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Log in on Reddy's Vault.")
}

#[get("/")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Welcome to ReddyVault...")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let localhost = Ipv4Addr::LOCALHOST;

    HttpServer::new(|| {
        App::new()
            .service(app)
            .service(login)
            .service(about)
            .wrap(Cors::permissive())
    })
    .bind((localhost, 8080))?
    .run()
    .await
}
