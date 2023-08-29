use std::net::TcpListener;
use std::path::Path;
use std::fs::{read_dir, File};
use std::collections::HashMap;
use std::str::FromStr;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use serde_json;

struct _PreImage {
    alg: String,
    msg: String
}

struct _MetaData {
    subject: String,
    policy: String,
    name: String,
    description: String,
    ticker: String,
    decimals: String,
    url: String,
    logo: String,
}

struct AppState {
    metadata: HashMap<String, serde_json::Value>,
}

async fn _greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok()
}

#[get("/metadata/{subject}")]
async fn all_properties(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // Root path
    // Find the file for the subject, read it and return the json
    let subject = path.into_inner();
    dbg!(&subject);
    let subject_file_path = format!("/Users/msch/src/cf/cardano-token-registry/mappings/{}.json", &subject);
    let subject_file_path = Path::new(&subject_file_path);
    let file = File::open(subject_file_path)
        .expect("Could not open file");
    let lejason:serde_json::Value = serde_json::from_reader(file)
        .expect("Could not load json");
    let meta = data.metadata.get(&subject).expect("Could not find it ");
    //println!("{:#?}", data.metadata);
    HttpResponse::Ok().json(meta)
}

// Run creates the server and returns a Result of that try
// Because creation of the server might fail during bind, where the ? indicates
// the possibility of an error bubbling up
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let mut metadatas: HashMap<String, serde_json::Value> = HashMap::new();
    // let _testfile = std::path::Path::new("/Users/msch/src/rust/token-api-z2prod/fed1c459a47cbff56bd7d29c2dde0de3e9bd15cee02b98622fce82f743617264616e6f476f6c64.json");
    let paths = read_dir("/Users/msch/src/cf/cardano-token-registry/mappings").unwrap();
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
            .app_data(web::Data::new(AppState {
                metadata: metadatas.clone(),
            })) // add shared state

            // Logger is a middleware that logs the requests, but its the env_logger
            // crate that writes them to stdout!
            .wrap(Logger::default())
            .route("/health", web::get().to(health))
            .service(all_properties)
        })
        //.bind("127.0.0.1:8000")?
        .listen(listener)?
        .run();

    Ok(server)
}

