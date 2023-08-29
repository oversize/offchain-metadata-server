use std::{net::TcpListener, path::Path};
use tokenapi::run;
use std::fs;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind random port");


    run(listener)?.await
}
