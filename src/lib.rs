use actix_web::{App, HttpServer, Result};
use std::collections::HashMap;
use std::fs::read_dir;
use std::net::TcpListener;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use log;
use serde_json;
use tokio::spawn;
use tokio_schedule::{every, Job};

mod api;
mod scheduler;

fn read_mappings(registry_path: PathBuf, mappings: &mut HashMap<String, serde_json::Value>) {
    let paths = read_dir(&registry_path).unwrap();
    //let mut mappings = mappings.lock().expect("Error acquiring mutex lock");
    for path in paths {
        let dir_entry = path.expect("File not found");
        let path = dir_entry.path();
        let json_data = std::fs::read_to_string(&path).expect("Json invalid");
        let stem_path = path.file_stem().unwrap();
        let stem_str = stem_path.to_str().unwrap();
        let key = String::from_str(stem_str).unwrap();
        //println!("key {:#?}", key);
        let json_data: serde_json::Value = serde_json::from_str(&json_data).expect("JSON invalid");
        mappings.insert(key, json_data);
    }
    log::info!("Read {} items", mappings.len());
}

// Run creates the server and returns a Result of that
pub fn run(listener: TcpListener, registry_path: PathBuf) -> Result<Server, std::io::Error> {
    let mut mappings: HashMap<String, serde_json::Value> = HashMap::new();
    read_mappings(registry_path, &mut mappings);
    let every_30_seconds = every(3)
        .seconds() // by default chrono::Local timezone
        .perform(|| async {
            println!("Every minute at 00'th and 30'th second");
            //read_mappings(registry_path, &mut mappings);
        });
    spawn(every_30_seconds);
    // let app_data = web::Data::new(api::AppState { metadata: mappings });
    let app_data = web::Data::new(api::AppState {
        mappings: mappings.clone(),
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
            .service(api::query)
    })
    .listen(listener)?
    .run();

    return Ok(server);
}
