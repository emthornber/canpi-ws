use crate::handlers::start_connection::*;
use actix_web::web;
use lazy_static::lazy_static;
use std::collections::HashMap;

const CBUS: &str = "/cbus";

lazy_static! {
    pub static ref ROUTE_DATA: HashMap<&'static str, String> = {
        let mut map = HashMap::new();

        #[allow(clippy::useless_format)]
        map.insert("root", format!("{CBUS}"));

        map
    };
}

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["root"].as_str())
            // .service(web::resource("").route(web::get().to(status_handler)))
            .service(web::resource(CBUS).route(web::get().to(start_connection))),
    );
}
