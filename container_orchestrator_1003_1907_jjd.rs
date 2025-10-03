use warp::Filter;
use std::error::Error;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define a simple in-memory storage to hold container information
    let containers: HashMap<String, String> = HashMap::new();

    // Define a filter to handle HTTP GET requests to list all containers
    let list_containers = warp::path("containers")
        .and(warp::get())
        .map(move || {
            let containers_list = containers.clone();
            Ok::<_, warp::Rejection>(
                json!({
                    "containers": containers_list.keys().cloned().collect::<Vec<String>>()
                }),
            )
        });

    // Define a filter to handle HTTP POST requests to add a new container
    let add_container = warp::path("containers")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |container: ContainerRequest| {
            let id = container.id.clone();
            containers.insert(id, container.image.clone());
            Ok::<_, warp::Rejection>(
                json!({
                    "message": "Container added successfully",
                    "container": {
                        "id": id,
                        "image": container.image,
                    },
                }),
            )
        });

    // Start the server
    warp::serve(list_containers.or(add_container)).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// Define a struct to represent a container
#[derive(Deserialize, Debug)]
struct ContainerRequest {
    id: String,
    image: String,
}

// Define a struct to represent a container response
#[derive(Serialize, Debug)]
struct ContainerResponse {
    id: String,
    image: String,
}

// Implement error handling for the container orchestrator
impl warp::reject::Reject for ContainerError {}

enum ContainerError {
    InvalidRequest,
    NoSuchContainer(String),
}

impl std::fmt::Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContainerError::InvalidRequest => write!(f, "Invalid request"),
            ContainerError::NoSuchContainer(ref id) => write!(f, "No such container: {}", id),
        }
    }
}

impl Error for ContainerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
