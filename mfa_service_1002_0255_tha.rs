 * This program demonstrates a simple MFA service structure that includes two factors of authentication:
 * a username and password check, and a one-time password (OTP) check.
 */

use warp::Filter;
use std::sync::Arc;
use std::error::Error;
use serde::{Serialize, Deserialize};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio::sync::Mutex;

// Define the User struct that holds user credentials and an OTP token.
#[derive(Clone)]
struct User {
    username: String,
    password: String,
    otp_token: Option<String>,
}

// Define the OTP store, which in a real-world scenario would be replaced by a secure database.
struct OtpStore {
    tokens: Arc<Mutex<std::collections::HashMap<String, String>>>,
}

impl OtpStore {
    fn new() -> Self {
        OtpStore {
            tokens: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    async fn generate_otp(&self, username: &str) -> Result<String, Box<dyn Error>> {
        let mut rng = thread_rng();
        let token: String = rng.sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let mut tokens = self.tokens.lock().await;
        tokens.insert(username.to_string(), token.clone());

        Ok(token)
    }

    async fn verify_otp(&self, username: &str, otp_token: &str) -> Result<bool, Box<dyn Error>> {
        let tokens = self.tokens.lock().await;
        tokens.get(username).map_or(Ok(false), |t| Ok(t == otp_token))
    }
}

// Define a simple user authentication request and response structure.
#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
    otp_token: Option<String>,
}

#[derive(Serialize)]
struct AuthResponse {
    success: bool,
    message: String,
}

// Define the routes for the authentication service.
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let otp_store = OtpStore::new();

    let generate_otp_route = warp::path("generate_otp")
        .and(warp::post())
        .and(warp::path::param::<String>().map(move |username| (username, otp_store.clone())))
        .and_then(handle_generate_otp);

    let verify_otp_route = warp::path("verify_otp")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::path::param::<String>().map(move |username| (username, otp_store.clone())))
        .and_then(handle_verify_otp);

    generate_otp_route.or(verify_otp_route)
}

// Handle the OTP generation endpoint.
async fn handle_generate_otp(username: String, otp_store: OtpStore) -> Result<impl warp::Reply, warp::Rejection> {
    match otp_store.generate_otp(&username).await {
        Ok(token) => Ok(warp::reply::json(&AuthResponse {
            success: true,
            message: format!("OTP token generated: {}", token),
        })),
        Err(e) => Ok(warp::reply::json(&AuthResponse {
            success: false,
            message: e.to_string(),
        })),
    }
}

// Handle the OTP verification endpoint.
async fn handle_verify_otp(request: AuthRequest, username: String, otp_store: OtpStore) -> Result<impl warp::Reply, warp::Rejection> {
    match otp_store.verify_otp(&username, &request.otp_token.unwrap_or_default()).await {
        Ok(true) => Ok(warp::reply::json(&AuthResponse {
            success: true,
            message: "OTP token verified successfully.".to_string(),
        })),
        Ok(false) => Ok(warp::reply::json(&AuthResponse {
            success: false,
            message: "Invalid OTP token.".to_string(),
        })),
        Err(e) => Ok(warp::reply::json(&AuthResponse {
            success: false,
            message: e.to_string(),
        })),
    }
}

#[tokio::main]
async fn main() {
    let _ = warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}
