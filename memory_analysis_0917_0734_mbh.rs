// memory_analysis.rs
use warp::Filter;
use std::sync::Arc;
use std::process::Command;
use tokio::process::Command as TokioCommand;
use serde::Serialize;
use serde_json;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::Filter;
use warp::reject::Reject;
use warp::Rejection;
use warp::reply::Reply;
use warp::http::Response;
use warp::reply::Response;
use warp::Filter;
use warp::Filter;
use warp::Filter;
use warp::Filter;

// Define a custom error type for our application
#[derive(Debug, Clone)]
struct MemoryUsageError;

// Implement Reject trait for MemoryUsageError to handle rejections in Warp
impl Reject for MemoryUsageError {}

// Define a structure to hold memory usage data
#[derive(Serialize)]
struct MemoryUsageData {
    total: u64,
    free: u64,
    available: u64,
    buffers: u64,
    cached: u64,
    swap_cached: u64,
    swap_total: u64,
    swap_free: u64,
}

// Function to run the 'free' command and get memory usage data
async fn get_memory_usage() -> Result<MemoryUsageData, MemoryUsageError> {
    let output = TokioCommand::new("free")
        .arg("-m")
        .output()
        .await
        .map_err(|_| MemoryUsageError)?;

    if !output.status.success() {
        return Err(MemoryUsageError);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Parse the output of the 'free' command
    let lines: Vec<&str> = output_str.lines().collect();
    let mem_info = lines[1].split_whitespace().collect::<Vec<&str>>();

    Ok(MemoryUsageData {
        total: mem_info[1].parse().unwrap(),
        free: mem_info[3].parse().unwrap(),
        available: mem_info[6].parse().unwrap(),
        buffers: mem_info[7].parse().unwrap(),
        cached: mem_info[8].parse().unwrap(),
        swap_cached: mem_info[10].parse().unwrap(),
        swap_total: mem_info[11].parse().unwrap(),
        swap_free: mem_info[12].parse().unwrap(),
    })
}

// Warp filter to handle GET requests for memory usage data
fn memory_usage_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("memory")
        .and(warp::get())
        .and_then(|| async move {
            get_memory_usage().map(Json)
                .map_err(|_| warp::reject::custom(MemoryUsageError))
        }).untuple_one()
}

// Main function to start the Warp server
#[tokio::main]
async fn main() {
    // Configure the Warp server
    let memory_usage_route = memory_usage_filter();

    // Start the server on localhost:3030
    warp::serve(memory_usage_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
