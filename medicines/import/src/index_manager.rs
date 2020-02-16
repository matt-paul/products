use reqwest;
use reqwest::StatusCode;
use std::collections::HashMap;

pub fn delete(blob_name: &str, verbosity: i8) -> Result<(), Box<dyn std::error::Error>> {
    if verbosity >= 2 {
        println!("---------------");
        println!("Deleting blob from index:");
        println!("{}", blob_name);
    }
    let search_service =
        std::env::var("SEARCH_SERVICE").expect("Set env variable SEARCH_SERVICE first!");
    let api_admin_key =
        std::env::var("API_ADMIN_KEY").expect("Set env variable API_ADMIN_KEY first!");
    let index_name = std::env::var("INDEX_NAME").expect("Set env variable INDEX_NAME first!");
    let uri = format!(
        "https://{}.search.windows.net/indexes/{}/docs/index?api-version=2017-11-11",
        search_service, index_name
    );
    let client = reqwest::blocking::Client::new();
    let params = [
        ("@search.action", "delete"),
        ("metadata_storage_name", blob_name),
    ];
    let mut map = HashMap::new();
    map.insert("@search.action", "delete");
    map.insert("metadata_storage_name", blob_name);
    let mut parent_map = HashMap::new();
    parent_map.insert("value", [map]);
    // let headers = reqwest::header::HeaderMap
    let mut res = client
        .post(&uri)
        .json(&parent_map)
        .header("Content-Type", "application/json")
        .header("api-key", api_admin_key)
        .send()?;
    // https://docs.rs/reqwest/0.10.0-alpha.2/reqwest/blocking/struct.Response.html
    match res.status() {
        StatusCode::OK => println!("Success!"),
        s => println!("{:?}", s),
    }
    Ok(())
}
