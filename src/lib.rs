pub mod configuratio;
pub mod routes;
pub mod startup;



use std::net::TcpListener;

use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::dev::Server;
use sqlx::PgConnection;
use crate::routes::subscribe;
// use reqwest::dns::Addrs;Addrs

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    if name.is_empty() {
        return HttpResponse::NotFound().body("404 Not Found");
    } else if name == "nina" {
        let path: PathBuf = "/home/bunny/Pictures/on.jpg".into();
        match NamedFile::open(path) {
            Ok(image) => return image.into_response(&req),
            Err(_) => return HttpResponse::NotFound().body("Image not found"),
        }
    }

    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(tcp_listener: TcpListener
          ) -> Result<Server,std::io::Error> {

    let server=HttpServer::new(move|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/Subscribe", web::post().to(subscribe))

        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))

    })
        .listen(tcp_listener)?
        .run();
    Ok(server)
}