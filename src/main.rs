use std::io::Error;
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::{Connection, PgConnection};

use newsletter::configuratio;
use newsletter::configuratio::get_config;
use newsletter::startup;
use newsletter::startup::run;

#[actix_web::main]  // Ensures an async runtime for Actix Web
  async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await.expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener,connection)?.await
  }


