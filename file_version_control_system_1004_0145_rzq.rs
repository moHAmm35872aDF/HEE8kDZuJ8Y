use warp::Filter;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use std::fs::File;

// Define a struct for file metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
struct FileVersion {
    version: i32,
    file_path: String,
    content: String,
}

// Define a struct for response
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Response {
    message: String,
}

// Define routes
fn main() {
    let file_version_route = warp::path("file_version")
        .and(warp::post())
        .and(with_file())
        .and_then(handle_file_version);

    let routes = file_version_route;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handler for file version control system
async fn handle_file_version(file: FileVersion) -> Result<impl warp::Reply, warp::Rejection> {
    let mut file_path = PathBuf::from(&file.file_path);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let dir_path = file_path.parent().unwrap().to_str().unwrap();
    let version = file.version;

    // Check if directory exists
    if !Path::new(dir_path).exists() {
        return Ok(warp::reply::json(&Response {
            message: format!("Directory '{}' does not exist", dir_path),
        }));
    }

    // Check if file exists
    if !Path::new(&file_path).exists() {
        return Ok(warp::reply::json(&Response {
            message: format!("File '{}' does not exist", file_path.display()),
        }));
    }

    // Create a new file with version number
    let new_file_name = format!("{}.v{}", file_name, version);
    let new_file_path = Path::new(dir_path).join(&new_file_name);

    // Write file content
    match fs::write(&new_file_path, &file.content) {
        Ok(_) => {
            return Ok(warp::reply::json(&Response {
                message: format!("File version {} created successfully", version),
            }));
        }
        Err(e) => {
            return Ok(warp::reply::json(&Response {
                message: format!("Failed to write file: {}