use log;
use std::{collections::HashMap, fs::read_dir, path::Path};

/// Function that reads the files in registry_path and updates the mappings
/// This function should add better error handling by returning a result so
/// the views can act accordingly!
pub fn read_mappings(registry_path: &Path, mappings: &mut HashMap<String, serde_json::Value>) {
    log::debug!(
        "Reading mappings from {}",
        registry_path.to_str().unwrap_or("?")
    );

    // Can we create a ReadDir iterator of the PathBug?
    let paths = read_dir(registry_path);

    let mut count_new_items = 0;

    // We know paths is not an error
    for path in paths.expect("Not a ReadDir Iterator") {
        let path = path.expect("Not a DirEntry").path();
        match path_as_key(&path) {
            None => log::warn!(
                "Unable to convert {} into valid mapping key. Ignoring.",
                path.to_str().unwrap_or("?")
            ),
            Some(key) => {
                if let Ok(raw_json) = std::fs::read_to_string(&path) {
                    if let Ok(json_data) = serde_json::from_str(&raw_json) {
                        if mappings.insert(key, json_data).is_none() {
                            count_new_items += 1;
                        }
                    }
                }
            }
        }
    }

    log::info!("Read {} new items", count_new_items);
}

// The key for mappings is the name of the json file on disk
fn path_as_key(path: &Path) -> Option<String> {
    Some(path.file_stem()?.to_str()?.to_string())
}
