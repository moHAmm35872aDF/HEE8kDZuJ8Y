 * Features:
 * - Checks if a given URL is reachable.
 * - Returns HTTP 200 if the URL is reachable, otherwise returns an error.
 *
 * Usage:
 * The service expects a GET request with a query parameter 'url' that contains the URL to check.
 */

use warp::Filter;
use std::net::IpAddr;
use std::str::FromStr;

// Define the route for checking network status
fn network_status_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("check")
        .and(warp::get())
        .and(warp::query::<IpAddr>())
        .and_then(handle_network_check)
}

// Handler function for network status check
async fn handle_network_check(ip: IpAddr) -> Result<impl warp::Reply, warp::Rejection> {
    // Attempt to connect to the IP address
    let result = std::net::TcpStream::connect((ip, 80)).await;

    match result {
        Ok(_) => Ok(warp::reply::json(&"reachable")),
        Err(_) => Ok(warp::reply::json(&"not_reachable")),
    }
}

#[tokio::main]
async fn main() {
    // Define the warp filter
    let routes = network_status_route();

    // Start the warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
