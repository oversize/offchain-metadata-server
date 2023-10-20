use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, HttpResponse, Responder};

use serde::Deserialize;
// use serde_json::de::Read;
use serde_json::{json, Value};

use log;

#[derive(Clone)]
pub struct AppMutState {
    pub mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub registry_path: String,
}

// Endpint for loadbalancer to check health of service
#[get("/health")]
pub async fn health() -> impl Responder {
    log::info!("health");
    HttpResponse::Ok()
}

/// Endpoint to trigger update of the data
#[get("/reread")]
pub async fn reread_mappings(app_data: web::Data<AppMutState>) -> impl Responder {
    read_mappings(app_data.registry_path.clone(), app_data.mappings.clone());
    HttpResponse::Ok().body("Reread contents")
}

/// Funcion that reads the files in registry_path and updates the mappings
/// This function should add better error handling by returning a result so
/// the views can act accordingly!
pub fn read_mappings(
    registry_path: String,
    mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
) {
    log::debug!("Reading mappings from {}", registry_path);

    // Can we create a PathBuf from registry_path?
    let path_buf = PathBuf::from_str(&registry_path);
    if path_buf.is_err() {
        log::error!("Not a string {}", &registry_path);
        return;
    }

    // Can we create a ReadDir iterator of the PathBug?
    let path_buf = path_buf.unwrap();
    let paths = read_dir(path_buf);

    let mut mtx = match mappings.lock() {
        Ok(mtx) => mtx,
        Err(e) => {
            log::error!("Error acquiring mutex lock! {}", e.to_string());
            return;
        }
    };

    // We know paths is not an error
    for path in paths.expect("Not a ReadDir Iterator") {
        let path = path.expect("Not a DirEntry").path();
        let stem_path = path.file_stem().expect("No file_stem");
        let stem_str = stem_path.to_str().expect("Failed creating str");
        // The key for hashmap is the name of the json file on disk
        let key = stem_str.to_string();
        if let Ok(raw_json) = std::fs::read_to_string(&path) {
            if let Ok(json_data) = serde_json::from_str(&raw_json) {
                mtx.insert(key, json_data);
            }
        }
    }
    log::info!("Read {} items", mtx.len());
}

/// Endpoint to retrieve a single subject
#[get("/metadata/{subject}")]
pub async fn single_subject(
    path: web::Path<String>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    // Grab mutex lock
    let mtx = match app_data.mappings.lock() {
        Ok(mtx) => mtx,
        Err(e) => {
            log::error!("Error acquiring mutex lock! {}", e.to_string());
            return HttpResponse::InternalServerError().body("");
        }
    };

    // extract subject_string from url path
    let subject_string = path.into_inner();

    // Grab subject for given subject_string or die
    let subject = match mtx.get(&subject_string) {
        Some(d) => d,
        None => {
            log::debug!("Subject not found {}", subject_string);
            return HttpResponse::NotFound().body("");
        }
    };

    HttpResponse::Ok().json(subject)
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
    // Grab mutex lock
    let mtx = match app_data.mappings.lock() {
        Ok(mtx) => mtx,
        Err(e) => {
            log::warn!("Error acquiring mutex lock! {}", e.to_string());
            return HttpResponse::InternalServerError().body("");
        }
    };

    // Extract subject_string from url path
    let subject_string = path.into_inner();

    // Grab subject for given subject_string or die
    let subject = match mtx.get(&subject_string) {
        Some(s) => s,
        None => {
            log::debug!("Subject not found {}", subject_string);
            return HttpResponse::NotFound().body("");
        }
    };

    HttpResponse::Ok().json(subject)
}

/// Endpoint to retrieve a specific property value for a given subject
/// The CIP Recommended /metadata/SUBJECT/property/NAME
/// But both other impementations have chosen to pick /metadata/SUBJECT/properties/NAME
/// https://tokens.cardano.org/metadata/5c4f08f47124b8e7ce9a4d0a00a5939da624cf6e533e1dc9de9b49c5556e636c6542656e6e793630/properties/logo
///
#[get("/metadata/{subject}/properties/{property}")]
pub async fn single_property(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    // Grab mutex lock
    let mtx = match app_data.mappings.lock() {
        Ok(mtx) => mtx,
        Err(e) => {
            log::error!("Error acquiring mutex lock! {}", e.to_string());
            return HttpResponse::InternalServerError().body("");
        }
    };

    // Extract subject_string and name from url path
    let (subject_string, property_name) = path.into_inner();

    // Grab subject for given subject_string or die
    let subject = match mtx.get(&subject_string) {
        Some(meta) => meta,
        None => {
            log::debug!("Subject not found {}", subject_string);
            return HttpResponse::NotFound().body("");
        }
    };

    // Grab property from subject or die
    let property_value = match subject.get(&property_name) {
        Some(v) => v,
        None => {
            // given key not found in metadata
            return HttpResponse::NotFound().body("");
        }
    };

    HttpResponse::Ok().json(json!({
        "subject": &subject, &property_name: property_value
    }))
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

    let mtx = match app_data.mappings.lock() {
        Ok(mtx) => mtx,
        Err(e) => {
            log::error!("Error acquiring mutex lock! {}", e.to_string());
            return HttpResponse::InternalServerError().body("");
        }
    };

    // Grab ref to properties so we can use it throughout the for loop below
    let properties = payload.properties.clone();
    log::debug!("Requested {} subjects", payload.subjects.len());
    if properties.is_some() {
        // as_ref() temporary references properties, so its not actually moved
        //   It needs to be used a little lower in the code
        log::debug!("   with {} properties", properties.as_ref().unwrap().len());
    }

    // return_subjects holds subjects that where requests and should be returned
    let mut return_subjects: Vec<Value> = Vec::new();
    for subject_string in payload.subjects.iter() {
        let subject = match mtx.get(subject_string) {
            Some(s) => s,
            None => {
                log::debug!("Subject not found {}", subject_string);
                continue;
            }
        };
        // Found subject, return full or only the given property
        match &properties {
            Some(property_strings) => {
                // Build a new subject only with given properties
                // Iterate over all given properties, search each in
                // the subject and add to newsubj if existing
                let mut newsubj: HashMap<&str, &Value> = HashMap::new();
                for ps in property_strings.iter() {
                    match subject.get(ps) {
                        Some(property) => {
                            newsubj.insert(ps, property);
                        },
                        None => {}
                    }
                }
                // What if none of the properties are found? Then you
                // should not have provided the list in the request in the
                // first place! However, this is where a correct implementation
                // of all_properties becomes important so the client can
                // check which to ask for.
                return_subjects.push(serde_json::json!(newsubj));
            },
            None => {
                // There are no properties provided, return whole subject
                return_subjects.push(subject.clone());
            }
        }
    }

    HttpResponse::Ok().json(json!({
        "subjects": return_subjects
    }))
}
