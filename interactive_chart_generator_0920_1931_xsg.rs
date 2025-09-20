 * interactive_chart_generator.rs
 *
 * This Rust program is designed to serve as an interactive chart generator using the Warp framework.
 * It handles HTTP requests to generate charts and provides a basic API for chart generation.
 */

use warp::Filter;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use thiserror::Error as ThisError;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::Reply;
use warp::Filter as WarpFilter;
use warp::Rejection;

// Define a custom error type for our application.
#[derive(Debug, ThisError)]
enum AppError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("Internal Server Error")]
    InternalServerError,
}

// Define a struct for the chart data that can be deserialized from the request.
#[derive(Deserialize, Serialize, Debug)]
struct ChartData {
    title: String,
    x_label: String,
    y_label: String,
    data: Vec<(f64, f64)>,
}

// Define a struct for the chart configuration.
#[derive(Deserialize, Serialize, Debug)]
struct ChartConfig {
    width: u32,
    height: u32,
}

// Define a filter to handle the GET request for the interactive chart.
fn chart_route() -> WarpFilter<impl Reply> {
    warp::path!("chart")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_chart_request)
}

// Define a handler function for the chart request.
async fn handle_chart_request(data: ChartData, config: ChartConfig) -> Result<impl Reply, Rejection> {
    if data.data.is_empty() {
        return Err(warp::reject::custom(AppError::InvalidRequest));
    }

    // TODO: Implement chart generation logic here.
    // For now, we just return a success message.
    Ok(warp::reply::json(&"Chart generated successfully"))
}

// Define a main function to run the Warp server.
#[tokio::main]
async fn main() {
    let chart_filter = chart_route();
    let routes = warp::service(chart_filter);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Implement the `Reject` trait for our custom error type.
impl Reject for AppError {}

// Implement the `fmt::Display` trait for our custom error type.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InvalidRequest => write!(f, "Invalid request"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

// Implement the `Error` trait for our custom error type.
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
