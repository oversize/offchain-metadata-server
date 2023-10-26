use env_logger::Env;
use std::{env, net::TcpListener};

use tokenapi::server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let registry_path = env::var("MAPPINGS").expect("Missing env var: MAPPINGS (Filepath)");
    let listen_address = env::var("LISTEN").expect("Missing env var: LISTEN (Port number)");

    log::info!("Listening on {}", &listen_address);

    let listener = TcpListener::bind(listen_address)
        .unwrap_or_else(|_| panic!("Failed to bind to LISTEN={:?}", env::var("LISTEN")));

    server::run(listener, registry_path)?.await
}
