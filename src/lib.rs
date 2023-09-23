use actix_web::{App, HttpServer, Result};
use std::collections::HashMap;
//use std::fs::read_dir;
//use std::str::FromStr;
use std::net::TcpListener;
// use std::path::PathBuf;

use std::sync::{Arc, Mutex};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
//use log;
use serde_json;

mod api;


// Run creates the server and returns a Result of that
pub fn run(listener: TcpListener, registry_path: String) -> Result<Server, std::io::Error> {
    let mappings: Arc<Mutex<HashMap<String, serde_json::Value>>> = Arc::new(Mutex::new(HashMap::new()));
    api::read_mappings(registry_path.clone(), mappings.clone());

    let app_data = web::Data::new(
        api::AppMutState {
            mappings: mappings.clone(),
            registry_path: registry_path.clone()
    });

    let server = HttpServer::new(move || {
        App::new()
            // Sharing the state with the handler
            // .app_data(app_data.clone())
            .app_data(app_data.clone())
            // Logger is a middleware that logs the requests, but its the env_logger
            // crate that writes them to stdout!
            .wrap(Logger::default())
            .service(api::health)
            .service(api::single_subject)
            .service(api::some_property)
            .service(api::all_properties)
            .service(api::query)
            .service(api::pong)
    })
    .listen(listener)?
    .run();

    return Ok(server);
}
