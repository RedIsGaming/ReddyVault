use std::net::Ipv4Addr;
use std::io::Result;
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use db::{self, role::Role, schema::sql_types::Roles};

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body("Who ReddyVault actually are")
}

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Log in on ReddyVault")
}

#[get("/")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Welcome to ReddyVault...")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let role = Role::User(Role::Default);

    HttpServer::new(|| {
        db::create("test", "test@test.com", "test", &role);
        db::read();
        
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
