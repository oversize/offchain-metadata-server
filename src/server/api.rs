use super::registry;
use actix_web::{get, post, web, HttpResponse, Responder};
use log;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct AppMutState {
    pub mappings: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub registry_path: PathBuf,
}

impl AppMutState {
    fn with_mappings<F>(&self, action: F) -> HttpResponse
    where
        F: FnOnce(&mut HashMap<String, serde_json::Value>) -> HttpResponse,
    {
        match self.mappings.lock() {
            Ok(mut mtx) => action(&mut mtx),
            Err(e) => {
                log::error!("Error acquiring mutex lock! {}", e.to_string());
                HttpResponse::InternalServerError().body("")
            }
        }
    }
}

// Endpoint for loadbalancer to check health of service
#[get("/health")]
pub async fn health() -> impl Responder {
    log::info!("health");
    HttpResponse::Ok()
}

/// Endpoint to trigger update of the data
#[get("/reread")]
pub async fn reread_mappings(app_data: web::Data<AppMutState>) -> impl Responder {
    app_data.with_mappings(|mappings| {
        registry::read_mappings(&app_data.registry_path, mappings);
        HttpResponse::Ok().body("Reread contents")
    })
}

/// Endpoint to retrieve a single subject
#[get("/metadata/{subject}")]
pub async fn single_subject(
    path: web::Path<String>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    app_data.with_mappings(|mappings| {
        // extract subject_string from url path
        let subject_string = path.into_inner();

        // Grab subject for given subject_string or die
        match mappings.get(&subject_string) {
            Some(subject) => HttpResponse::Ok().json(subject),
            None => not_found(&subject_string),
        }
    })
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
    app_data.with_mappings(|mappings| {
        // Extract subject_string from url path
        let subject_string = path.into_inner();

        // Grab subject for given subject_string or die
        match mappings.get(&subject_string) {
            Some(subject) => HttpResponse::Ok().json(subject),
            None => not_found(&subject_string),
        }
    })
}

/// Endpoint to retrieve a specific property value for a given subject
/// The CIP Recommended /metadata/SUBJECT/property/NAME
/// But both other impementations have chosen to pick /metadata/SUBJECT/properties/NAME
/// https://tokens.cardano.org/metadata/5c4f08f47124b8e7ce9a4d0a00a5939da624cf6e533e1dc9de9b49c5556e636c6542656e6e793630/properties/logo
#[get("/metadata/{subject}/properties/{property}")]
pub async fn single_property_by_other_impl(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    single_property(path, app_data).await
}

#[get("/metadata/{subject}/property/{property}")]
pub async fn single_property_by_spec(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    single_property(path, app_data).await
}

async fn single_property(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppMutState>,
) -> impl Responder {
    app_data.with_mappings(|mappings| {
        // Extract subject_string and name from url path
        let (subject_string, property_name) = path.into_inner();

        // Grab subject for given subject_string or die
        let subject = match mappings.get(&subject_string) {
            Some(meta) => meta,
            None => return not_found(&subject_string),
        };

        // Grab property from subject or die
        let property_value = match subject.get(&property_name) {
            Some(v) => v,
            None => return not_found(&property_name),
        };

        HttpResponse::Ok().json(json!({
            "subject": &subject, &property_name: property_value
        }))
    })
}

/// A query payload for the batch query endpoint
#[derive(Deserialize)]
pub struct Query {
    subjects: Vec<String>,
    properties: Option<Vec<String>>,
}

/// Endpoint for batch requesting multiple subjects at once
/// If the payload holds 'properties' the subject should be narrowed down
/// to only these properties
#[post("/metadata/query")]
pub async fn query(payload: web::Json<Query>, app_data: web::Data<AppMutState>) -> impl Responder {
    app_data.with_mappings(|mappings| {
        // Grab ref to properties so we can use it throughout the for loop below
        log::debug!("Requested {} subjects", payload.subjects.len());
        if let Some(ref properties) = payload.properties {
            log::debug!("   with {} properties", properties.len());
        }

        // return_subjects holds subjects that where requests and should be returned
        let mut return_subjects: Vec<Value> = Vec::new();
        for subject_string in payload.subjects.iter() {
            let subject = match mappings.get(subject_string) {
                Some(s) => s,
                None => {
                    log::debug!("Subject not found {}", subject_string);
                    continue;
                }
            };

            // Found subject, return full or only the given property
            match &payload.properties {
                Some(property_strings) => {
                    // Build a new subject only with given properties
                    // Iterate over all given properties, search each in
                    // the subject and add to newsubj if existing
                    let mut newsubj: HashMap<&str, &Value> = HashMap::new();
                    for k in property_strings.iter() {
                        if let Some(v) = subject.get(k) {
                            newsubj.insert(k, v);
                        }
                    }

                    // What if none of the properties are found? Then you
                    // should not have provided the list in the request in the
                    // first place! However, this is where a correct implementation
                    // of all_properties becomes important so the client can
                    // check which to ask for.
                    return_subjects.push(serde_json::json!(newsubj));
                }
                None => {
                    // There are no properties provided, return whole subject
                    return_subjects.push(subject.clone());
                }
            }
        }

        HttpResponse::Ok().json(json!({
            "subjects": return_subjects
        }))
    })
}

fn not_found(key: &str) -> HttpResponse {
    log::debug!("Subject or property not found {}", key);
    HttpResponse::NotFound().body("")
}
