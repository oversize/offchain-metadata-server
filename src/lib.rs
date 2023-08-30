use std::net::TcpListener;
use std::path::PathBuf;
use std::fs::read_dir;
use std::collections::HashMap;
use std::str::FromStr;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use serde_json;

mod api;

// Run creates the server and returns a Result of that try
// Because creation of the server might fail during bind, where the ? indicates
// the possibility of an error bubbling up
pub fn run(listener: TcpListener, mappings: PathBuf) -> Result<Server, std::io::Error> {
    let mut metadatas: HashMap<String, serde_json::Value> = HashMap::new();
    // let _testfile = std::path::Path::new("/Users/msch/src/rust/token-api-z2prod/fed1c459a47cbff56bd7d29c2dde0de3e9bd15cee02b98622fce82f743617264616e6f476f6c64.json");
    println!("{:?}", mappings);
    let paths = read_dir(&mappings).unwrap();
    for path in paths {
        let dir_entry = path.expect("File not found");
        let path = dir_entry.path();
        let json_data = std::fs::read_to_string(&path).expect("Json invalid");
        let stem_path = path.file_stem().unwrap();
        let stem_str = stem_path.to_str().unwrap();
        let key = String::from_str(stem_str).unwrap();
        //println!("key {:#?}", key);
        let metadata_json:serde_json::Value = serde_json::from_str(&json_data).expect("JSON invalid");
        metadatas.insert(key, metadata_json);
    }

    let server = HttpServer::new(move || {
        App::new()
            // Sharing the state with the handler
            .app_data(web::Data::new(api::AppState {
                metadata: metadatas.clone()
            })) // add shared state

            // Logger is a middleware that logs the requests, but its the env_logger
            // crate that writes them to stdout!
            .wrap(Logger::default())
            .service(api::health)
            .service(api::single_subject)
            .service(api::all_properties)
            .service(api::query)
        })
        //.bind("127.0.0.1:8000")?
        .listen(listener)?
        .run();

    Ok(server)
}

