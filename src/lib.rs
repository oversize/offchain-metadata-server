use actix_web::{App, HttpServer, Result};
use std::collections::HashMap;
use std::fs::read_dir;
use std::str::FromStr;
use std::net::TcpListener;
use std::path::PathBuf;

use std::sync::{Arc, Mutex};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use log;
use serde_json;

mod api;


// Run creates the server and returns a Result of that
pub fn run(listener: TcpListener, _registry_path: PathBuf) -> Result<Server, std::io::Error> {
    let mappings: Arc<Mutex<HashMap<String, serde_json::Value>>> = Arc::new(Mutex::new(HashMap::new()));
    let registry_path = "/Users/msch/src/cf/cardano-token-registry/mappings";
    api::read_mappings(registry_path, mappings.clone());

    let app_data = web::Data::new(
        api::AppMutState {
            mappings: mappings.clone(),
            registry_path
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
            .service(api::all_properties)
            .service(api::pong)
            .service(api::query)
    })
    .listen(listener)?
    .run();

    return Ok(server);
}
