use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

pub struct AppState {
    pub metadata: HashMap<String, serde_json::Value>,
}

struct _PreImage {
    alg: String,
    msg: String,
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

/// A query payload for the batch query endpoint
#[derive(Deserialize)]
pub struct Query {
    subjects: Vec<String>,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok()
}

/// Endpoint to retrieve a single subject
#[get("/metadata/{subject}")]
pub async fn single_subject(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // Extract subject from path
    let subject = path.into_inner();
    // Extract Value and return to json
    let meta = data.metadata.get(&subject).expect("Could not find it ");
    //println!("{:#?}", data.metadata);
    HttpResponse::Ok().json(meta)
}

/// Endpoint to retrieve all porperty names for a given subject
#[get("/metadata/{subject}/properties")]
pub async fn all_properties() -> impl Responder {
    HttpResponse::Ok()
}

/// Endpoint to retrieve a specific property value for a given subject
#[get("/metadata/{subject}/property/{name}")]
pub async fn some_property(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (subject, _name) = path.into_inner();
    let meta = data.metadata.get(&subject).expect("Could not find it ");
    println!("{:#?}", meta);
    todo!("Thats actually not finished yet");
    HttpResponse::Ok().json(meta)
}


/// Endpoint for batch requesting multiple subjects at once
///
/// If Content-Type is not 'application/json' return 415
#[post("/metadata/query")]
pub async fn query(payload: web::Json<Query>, data: web::Data<AppState>) -> impl Responder {
    println!("{:#?}", payload.subjects);
    let mut subjects: Vec<serde_json::Value> = Vec::new();

    //println!("{:#?}", payload.properties);
    for subject in payload.subjects.iter() {
        let meta = data.metadata.get(subject).expect("Could not find it ");
        subjects.push(meta.to_owned())
    }
    let out = serde_json::json!({
        "subjects": subjects
    });
    HttpResponse::Ok().json(out)
}
