use actix::Actor;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use simple_logger::SimpleLogger;
use std::process;
use time::macros::format_description;

mod errors;
mod handlers;
mod lobby;
mod messages;
mod routes;
mod validation;
mod ws;

use lobby::Lobby;
use routes::*;
use validation::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .env()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();
    log::info!("canpi websocket started");

    if let Ok(canpi_cfg) = CanpiConfig::new() {
        let chat_server = Lobby::default().start(); //create and spin up a lobby
        let app = move || {
            App::new()
                .configure(general_routes)
                .app_data(Data::new(chat_server.clone())) //register the lobby
        };

        // Start HTTP Server
        let host_port = canpi_cfg.host_port.unwrap();
        log::info!("Listening on: {}", host_port);
        HttpServer::new(app).bind(&host_port)?.run().await
    } else {
        log::error!("EV contents failed validation - exiting ...");
        process::exit(1);
    }
}
