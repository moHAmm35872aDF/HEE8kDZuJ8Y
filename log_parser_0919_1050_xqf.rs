use warp::Filter;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use warp::reject;
use warp::reply::Reply;
use warp::http::StatusCode;
use warp::filters::path::end;
use warp::filters::method::get;
use warp::filters::path::full;
use warp::filters::body::content_length_limit;
use warp::filters::body::bytes;
use warp::filters::extractors::body::Json;
use serde::Deserialize;
use serde_json::json;

// Define the structure of the request body
#[derive(Deserialize, Debug)]
struct LogRequest {
    file_path: String,
}

// Function to parse a log file
async fn parse_log(file_path: String) -> Result<impl Reply, warp::Rejection> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(reject::not_found());
    }
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(reject::reject()),
    };
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.map_err(|e| reject::custom(e))?;
        lines.push(line);
    }
    Ok(warp::reply::json(&lines))
}

// Define the route
fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    get()
        .and(end()) // Matches any GET request to the root path
        .and(warp::path("parse_log")) // Matches the path /parse_log
        .and(content_length_limit(1024 * 1024)) // Limit the body size to 1MB
        .and(bytes()) // Extracts the request body as bytes
        .and_then(|body: bytes::Bytes| async move {
            let log_request: LogRequest = match serde_json::from_slice(&body) {
                Ok(log_request) => log_request,
                Err(_) => return Err(reject::reject()),
            };
            parse_log(log_request.file_path).await
        })
}

// Main function to start the server
#[tokio::main]
async fn main() {
    println!("Starting the log parser server...");
    let api = routes();
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
