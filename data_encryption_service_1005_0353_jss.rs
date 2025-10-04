use warp::Filter;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs;
use tokio::io::AsyncReadExt;
use openssl::symm::{encrypt_aead, Cipher};
use openssl::error::ErrorStack;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Error;

// Define a struct for the incoming request data.
#[derive(Deserialize, Serialize)]
struct EncryptRequest {
    data: String,
    key: String,
}

// Define a struct for the response data.
#[derive(Deserialize, Serialize)]
struct EncryptResponse {
    encrypted_data: String,
}

// Define an error type for encryption errors.
#[derive(Debug)]
struct EncryptionError(String);

// Implement Reject for EncryptionError to handle errors in Warp.
impl Reject for EncryptionError {}

// The key for encryption should be loaded from a secure source.
// In this example, we are using a hardcoded key for simplicity.
// In a real-world scenario, consider using a key management solution.
const ENCRYPTION_KEY: &str = "your_encryption_key_here";

// The function to encrypt the data.
async fn encrypt_data(request: EncryptRequest) -> Result<impl reply::Reply, EncryptionError> {
    let data = request.data;
    let key = request.key;

    // Check if the key matches the expected key.
    if key != ENCRYPTION_KEY {
        return Err(EncryptionError("Invalid encryption key.".to_string()));
    }

    // Perform encryption using OpenSSL.
    let cipher = Cipher::aes_256_gcm();
    let mut crypter = encrypt_aead(cipher, key.as_bytes(), b"").map_err(|error| EncryptionError(error.to_string()))?;
    let mut encrypted_data = vec![0; data.len() + 16];
    let count = crypter.update(data.as_bytes(), &mut encrypted_data).map_err(|error| EncryptionError(error.to_string()))?;
    crypter.finalize().map_err(|error| EncryptionError(error.to_string()))?;
    encrypted_data.truncate(count);

    Ok(json!(EncryptResponse {
        encrypted_data: base64::encode(&encrypted_data),
    })).into_response()
}

// The main function to set up the Warp server.
#[tokio::main]
async fn main() {
    // Define the API endpoint for encryption.
    let encryption_route = warp::post()
        .and(warp::path("encrypt"))
        .and(warp::body::json())
        .and_then(encrypt_data);

    // Start the Warp server.
    warp::serve(encryption_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
