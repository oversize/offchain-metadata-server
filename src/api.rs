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
// use std::error::Error;
// use std::io::Error;

#[derive(Clone)]
pub struct _AppState {
    pub mappings: HashMap<String, serde_json::Value>,
}

pub struct AppMutState {
    pub mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub registry_path: String,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok()
}

/// Funcion that reads the files in registry_path and updates the mappings
/// This function should add better error handling by returning a result so
/// the views can act accordingly
pub fn read_mappings(
    registry_path: String,
    mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
) {
    let paths = read_dir(PathBuf::from_str(&registry_path).unwrap()).unwrap();
    let mut mtx = mappings.lock().expect("Error acquiring mutex lock");
    for path in paths {
        let dir_entry = path.expect("Path invalid");
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
    app_data: web::Data<AppMutState>,
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
/// While the CIP says it should return a list of strings that are the properties
/// of given subject, the currently live implementation does not do that.
/// So this view will do the same (as does the implementation i am trying to replace)
#[get("/metadata/{subject}/properties")]
pub async fn all_properties(
    path: web::Path<String>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    let subject = path.into_inner();
    let mtx = app_data
        .mappings
        .lock()
        .expect("Error acquiring mutex lock");
    match mtx.get(&subject) {
        Some(d) => {
            log::debug!("Found Value for {}", subject);
            return HttpResponse::Ok().json(d);
        }
        None => {
            log::debug!("No Value found for {}", subject);
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
    app_data: web::Data<AppMutState>,
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
pub async fn pong(app_data: web::Data<AppMutState>) -> impl Responder {
    read_mappings(app_data.registry_path.clone(), app_data.mappings.clone());
    HttpResponse::Ok()
}

/// A query payload for the batch query endpoint
#[derive(Deserialize)]
pub struct Query{
    subjects: Vec<String>,
    properties: Option<Vec<String>>,
}

/// Endpoint for batch requesting multiple subjects at once
/// If the payload holds 'properties' the subject should be narrowed down
/// to only these properties
#[post("/metadata/query")]
pub async fn query(
    payload: web::Json<Query>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    // subjects holds subjects that where requests and should be returned
    let mut subjects: Vec<Value> = Vec::new();
    let mtx = app_data
        .mappings
        .lock()
        .expect("Error acquiring mutex lock");
    // Grab ref to properties so we can use it throughout the for loop below
    let properties = payload.properties.clone();
    log::debug!("Requested {} subjects", payload.subjects.len());
    if properties.is_some() {
        // as_ref() temporary references properties, so its not actually moved
        log::debug!("   with {} properties", properties.as_ref().unwrap().len());
    }

    for subject in payload.subjects.iter() {
        // Find subject in mappings or do nothing
        match mtx.get(subject) {
            Some(subj) => {
                // If there are properties given in the payload, only return
                // these for each subject, if not return the whole subject
                match &properties {
                    Some(props) => {
                        // Build a new subject only with given properties
                        let mut newsubj: HashMap<&str, &Value> = HashMap::new();
                        for p in props.iter() {
                            let value = subj.get(p);
                            if value.is_some() {
                                newsubj.insert(p, value.unwrap());
                            }
                        }
                        subjects.push(serde_json::json!(newsubj));
                    },
                    None => {
                        // There are no properties given, return whole subject
                        subjects.push(subj.clone());
                    }
                }
            },
            None => {
                log::debug!("Subject not found {}", subject);
            }
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
