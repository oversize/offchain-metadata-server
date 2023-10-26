use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer, Result};
use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
};

mod api;

pub fn run(listener: TcpListener, registry_path: String) -> Result<Server, std::io::Error> {
    let mappings: Arc<Mutex<HashMap<String, serde_json::Value>>> =
        Arc::new(Mutex::new(HashMap::new()));
    api::read_mappings(registry_path.clone(), mappings.clone());

    let app_data = web::Data::new(api::AppMutState {
        mappings: mappings.clone(),
        registry_path: registry_path.clone(),
    });

    let server = HttpServer::new(move || {
        App::new()
            // Sharing the state with the handler
            .app_data(app_data.clone())
            // Logger is a middleware that logs the requests, but its the env_logger
            // crate that writes them to stdout!
            .wrap(Logger::default())
            .service(api::health)
            .service(api::single_subject)
            .service(api::single_property)
            .service(api::all_properties)
            .service(api::query)
            .service(api::reread_mappings)
    })
    .listen(listener)?
    .run();

    Ok(server)
}