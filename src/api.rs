use actix_web::{get, post, web, HttpResponse, Responder};
use log;
use serde::Deserialize;
use serde_json::{self, Value};
use std::collections::HashMap;
// use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct _AppState {
    pub mappings: HashMap<String, serde_json::Value>,
}

pub struct AppMutState<'a> {
    pub mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub registry_path: &'a str,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok()
}

pub fn read_mappings(
    registry_path: &str,
    mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
) {
    let paths = read_dir(PathBuf::from_str(&registry_path).unwrap()).unwrap();
    let mut mtx = mappings.lock().expect("Error acquiring mutex lock");
    for path in paths {
        let dir_entry = path.expect("File not found");
        let path = dir_entry.path();
        let json_data = std::fs::read_to_string(&path).expect("Json invalid");
        let stem_path = path.file_stem().unwrap();
        let stem_str = stem_path.to_str().unwrap();
        let key = String::from_str(stem_str).unwrap();
        //println!("key {:#?}", key);
        let json_data: serde_json::Value = serde_json::from_str(&json_data).expect("JSON invalid");
        mtx.insert(key, json_data);
    }
    log::info!("Read {} items", mtx.len());
}

/// Endpoint to retrieve a single subject
#[get("/metadata/{subject}")]
pub async fn single_subject(
    path: web::Path<String>,
    app_data: web::Data<AppMutState<'_>>,
) -> impl Responder {
    let subject = path.into_inner();
    //let mappings = app_data.mappings;//.lock().expect("Error acquiring mutex lock");
    match app_data.mappings.lock().expect("Error").get(&subject) {
        Some(d) => {
            return HttpResponse::Ok().json(d);
        }
        None => {
            log::debug!("Nothing found for {}", subject);
            return HttpResponse::NotFound().body("");
        }
    };
}

/// Endpoint to retrieve all porperty names for a given subject
///
#[get("/metadata/{subject}/properties")]
pub async fn all_properties(
    path: web::Path<String>,
    app_data: web::Data<AppMutState<'_>>,
) -> impl Responder {
    let subject = path.into_inner();
    let mtx = app_data
        .mappings
        .lock()
        .expect("Error acquiring mutex lock");
    match mtx.get(&subject) {
        Some(d) => {
            // d is the serde_value
            let a = d.as_array();
            println!("{:?}", a);
            let name = d.get("name").unwrap().get("value").unwrap();
            log::debug!("Found {} for {}", name, subject);

            return HttpResponse::Ok().json(d);
        }
        None => {
            log::debug!("Nothing found for {}", subject);
            return HttpResponse::NotFound().body("");
        }
    };
}

/// Endpoint to retrieve a specific property value for a given subject
/// The CIP Recommended /metadata/SUBJECT/property/NAME
/// But both other impementations have chosen to pick /metadata/SUBJECT/properties/NAME
/// https://tokens.cardano.org/metadata/5c4f08f47124b8e7ce9a4d0a00a5939da624cf6e533e1dc9de9b49c5556e636c6542656e6e793630/properties/logo
///
#[get("/metadata/{subject}/properties/{name}")]
pub async fn some_property(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppMutState<'_>>,
) -> impl Responder {
    let (subject, _name) = path.into_inner();
    let mtx = app_data
        .mappings
        .lock()
        .expect("Error acquiring mutex lock");
    let meta = mtx.get(&subject).expect("Could not find it ");
    log::debug!("{:#?}", meta);
    HttpResponse::Ok().json(meta)
}

/// Endpoint to trigger update of the data
#[get("/pong")]
pub async fn pong(app_data: web::Data<AppMutState<'_>>) -> impl Responder {
    // let registry_path = PathBuf::from_str("/Users/msch/src/cf/cardano-token-registry/mappings").expect("Doh'");
    read_mappings(app_data.registry_path, app_data.mappings.clone());
    HttpResponse::Ok()
}

/// A query payload for the batch query endpoint
#[derive(Deserialize)]
pub struct Query {
    subjects: Vec<String>,
    properties: Option<Vec<String>>,
}

/// Endpoint for batch requesting multiple subjects at once
///
/// If Content-Type is not 'application/json' return 415
/// I am accepting serder_json::Value here because i currently dont know
/// how to provide the struct type with optional values (properties might or
/// might not be in the payload). And if its in the struct but not in
/// the request the request fails -> bad request, coz its missing.
///
/// Given the simplicity of the pazload however, its ok to deal with it in the
/// handler manually though.
#[post("/metadata/query")]
pub async fn query(
    payload: web::Json<Query>,
    app_data: web::Data<AppMutState<'_>>,
) -> impl Responder {
    println!("{:#?}", payload.subjects);
    let mut subjects: Vec<Value> = Vec::new();
    //let mappings = app_data.mappings.lock().expect("Error acquiring mutex lock");

    for subject in payload.subjects.iter() {
        log::debug!("subject into vec");
        let mtx = app_data.mappings.lock().expect("Erro");
        let subj = mtx.get(subject);
        if subj.is_some() {
            let subj = subj.unwrap();
            // Either return whole thing or only fields given by properties
            if payload.properties.is_none() {
                subjects.push(subj.to_owned());
            } else {
                let props = payload.properties.as_ref().unwrap();
                let mut newsubj: HashMap<&String, &Value> = HashMap::new();
                for p in props.iter() {
                    let value = subj.get(p);
                    if value.is_some() {
                        newsubj.insert(p, value.unwrap());
                    }
                }
                subjects.push(serde_json::json!(newsubj));
                // Only parse out the fields given in properties
            }
        } else {
            log::info!("No data found for {}", subject);
        }
    }

    // let mut subjects: Vec<serde_json::Value> = Vec::new();
    //for subject in subjects.iter() {
    //    let meta = data.metadata.get(subject).expect("Could not find it ");
    //    subjects.push(meta.to_owned())
    //}
    let out = serde_json::json!({
        "subjects": subjects
    });
    HttpResponse::Ok().json(out)
}
