use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

// Define a struct to represent the Data Lineage
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataLineage {
    source: String,
    transformations: Vec<String>,
    sink: String,
}

// Define a struct to hold the lineage data in memory
struct LineageStore {
    lineage_data: Arc<Mutex<Vec<DataLineage>>>,
}

impl LineageStore {
    // Add a new DataLineage to the store
    fn add_lineage(&self, lineage: DataLineage) -> Result<(), String> {
        let mut data = self.lineage_data.lock().map_err(|e| e.to_string())?;
        data.push(lineage);
        Ok(())
    }

    // Get all DataLineages from the store
    fn get_lineages(&self) -> Vec<DataLineage> {
        self.lineage_data.lock().map_err(|e| vec![]).unwrap_or_else(|_| vec![])
    }
}

// Define a filter to handle POST requests for adding new data lineages
fn add_lineage_route(store: Arc<LineageStore>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("lineage"))
        .and(warp::body::json()) // Expect JSON body
        .and(with_lineage_store(store)) // Use the LineageStore
        .and_then(|lineage: DataLineage, store: Arc<LineageStore>| async move {
            store.add_lineage(lineage).map_err(|e| warp::reject::custom(e))?;
            warp::reply::json(&"Lineage added successfully")
        }).recover(handle_rejection)
}

// Define a filter to handle GET requests for retrieving all data lineages
fn get_lineages_route(store: Arc<LineageStore>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("lineages"))
        .and(with_lineage_store(store)) // Use the LineageStore
        .and_then(|store: Arc<LineageStore>| async move {
            warp::reply::json(&store.get_lineages())
        }).recover(handle_rejection)
}

// Helper function to extract the LineageStore from the warp::Filter
fn with_lineage_store(store: Arc<LineageStore>) -> impl Filter<Extract = (Arc<LineageStore>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}

// Function to handle rejections
fn handle_rejection(err: warp::Rejection) -> impl warp::Reply {
    eprintln!("Rejection: {:?}