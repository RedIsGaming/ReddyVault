use std::net::Ipv4Addr;
use std::io::Result;
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Reddy's Vault is succesfully connected!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let localhost = Ipv4Addr::LOCALHOST;

    HttpServer::new(|| {
        App::new()
            .service(app)
            .wrap(Cors::permissive())
    })
    .bind((localhost, 8080))?
    .run()
    .await
}
