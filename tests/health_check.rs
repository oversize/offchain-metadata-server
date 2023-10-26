//! tests/health_check.rs
use std::net::TcpListener;
use tokenapi::server;

// spawn_app runs the application in the background so we can run tests
// against it. Should that server fail to create there is no need to
// handle that error, just let it panic. If created, the function
// returns the address of that temporary server in the form of ip:port
// That is usefull in the individual tests, to give the client an address
// to work against.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let ip = listener.local_addr().unwrap().ip();
    let mappings = String::from("./registry/mappings");

    let server = server::run(listener, mappings).expect("Failed to create server");

    // tokio spawn takes a future and hands it over to its runtime for
    // continious polling
    tokio::spawn(server);
    format!("http://{}:{}", ip, port)
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works() {
    // spawn_app runs the app in the background
    let address = spawn_app();
    let client = reqwest::Client::new();
    // Run actual test client
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subject_endpoint_returns_404() {
    // spawn_app runs the app in the background
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/metadata/DOESNOTEXIST", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 404);
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subject_endpoint_returns_200() {
    // spawn_app runs the app in the background
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        // hash must be from registry_data folder
        .get(&format!(
            "{}/metadata/782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574",
            &address
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn query_endpoints_returns_400() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/metadata/query", &address))
        .send()
        .await
        .expect("Dies!");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
}
