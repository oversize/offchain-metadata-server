use env_logger::Env;
use std::env;
use std::net::TcpListener;

use log;

use tokenapi::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let registry_path = String::from(env::var("MAPPINGS").expect("You need to set MAPPINGS"));
    let listen_address = env::var("LISTEN").expect("You need to set LISTEN");
    log::info!("Listening on {}", &listen_address);
    let listener = TcpListener::bind(listen_address)
        .expect(&format!("Failed to bind to {:?}", env::var("LISTEN")));
    run(listener, registry_path)?.await
}
