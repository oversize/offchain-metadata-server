use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use log;

pub struct AppState {
    pub metadata: HashMap<String, serde_json::Value>,
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
    match data.metadata.get(&subject) {
        Some(d) => {
            // todo: this may panic!
            let name = d.get("name").unwrap().get("value").unwrap();
            log::debug!("Found {} for {}", name, subject);
            return HttpResponse::Ok().json(d);
        },
        None => {
            log::debug!("Nothing found for {}", subject);
            return HttpResponse::NotFound().body("");
        }
    };
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
    log::debug!("{:#?}", meta);
    HttpResponse::Ok().json(meta)
}

/// A query payload for the batch query endpoint
#[derive(Deserialize)]
pub struct Query {
    subjects: Vec<String>,
    properties: Option<Vec<String>>
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
pub async fn query(payload: web::Json<Query>, data: web::Data<AppState>) -> impl Responder {
    println!("{:#?}", payload.subjects);
    let mut subjects: Vec<Value> = Vec::new();
    for subject in payload.subjects.iter() {
        log::debug!("subject into vec");
        let subj = data.metadata.get(subject);
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
