use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use crate::health_check;
use crate::routes::subscribe;
use sqlx::PgConnection;



pub fn run(tcp_listener: TcpListener
, connection: PgConnection) -> Result<Server,std::io::Error> {
    let connection = web::Data::new(connection);
    let server=HttpServer::new(move|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/Subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))

    })
        .listen(tcp_listener)?
        .run();
    Ok(server)
}
