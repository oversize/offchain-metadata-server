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

    let _testfile = std::path::Path::new("/Users/msch/src/rust/token-api-z2prod/fed1c459a47cbff56bd7d29c2dde0de3e9bd15cee02b98622fce82f743617264616e6f476f6c64.json");
    //println!("{:#?}", v);
    //let paths = fs::read_dir("/Users/msch/src/cf/cardano-token-registry/mappings").unwrap();
    //for path in paths {
    //    let file = path.expect("File not found");
    //    let path = file.path();
    //    println!("File {}", path.display());
    //    println!("json {:?}", d)
    //}

    run(listener)?.await
}
