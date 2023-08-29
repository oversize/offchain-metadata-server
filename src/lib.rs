use std::net::TcpListener;
use std::path::Path;
use std::fs::File;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

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

async fn _greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok()
}

#[get("/metadata/{subject}")]
async fn all_properties(path: web::Path<String>) -> impl Responder {
    // Find the file for the subject, read it and return the json
    let subject = path.into_inner();
    dbg!(&subject);
    let subject_file_path = format!("/Users/msch/src/rust/token-api-z2prod/{}.json", &subject);
    let subject_file_path = Path::new(&subject_file_path);
    let file = File::open(subject_file_path)
        .expect("Could not open file");
    let lejason:serde_json::Value = serde_json::from_reader(file)
        .expect("Could not load json");
    println!("{:#?}", lejason);

    HttpResponse::Ok().json(lejason)
}

// Run creates the server and returns a Result of that try
// Because creation of the server might fail during bind, where the ? indicates
// the possibility of an error bubbling up
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
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



