use warp::Filter;
use warp::http::StatusCode;
use warp::reject::{Reject, ValidationError};
use warp::reply::Reply;
use serde::Deserialize;
use serde_json::json;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;
use diesel::sqlite::SqliteConnection;
use diesel::result::QueryResult;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

// Define the error types
#[derive(Debug)]
struct DatabaseError(String);

impl Reject for DatabaseError {}

// Define the input data structure
#[derive(Debug, Deserialize)]
struct UserInput {
    username: String,
}

// Define the main function
#[tokio::main]
async fn main() {
    // Define the route
    let api = warp::post()
        .and(warp::path("query"))
        .and(warp::body::json())
        .and(with_db())
        .and_then(handle_query);

    // Start the server
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

// Function to handle the query
async fn handle_query(input: UserInput, db: PooledConnection<ConnectionManager<PgConnection>>) -> Result<impl Reply, warp::Rejection> {
    // Use prepared statements to prevent SQL injection
    let result: QueryResult<Vec<String>> = diesel::sql_query("SELECT * FROM users WHERE username = $1")
        .bind::<String, diesel::pg::Pg>(input.username)
        .load(&*db);

    // Handle any database errors
    match result {
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(e) => Err(warp::reject::custom(DatabaseError(e.to_string())).into()),
    }
}

// Function to create a shared database connection pool
fn with_db() -> impl Filter<Extract = (PooledConnection<ConnectionManager<PgConnection>>,), Error = warp::Rejection> + Clone {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    warp::any().map(move || pool.get().expect("Failed to get db connection from pool."))
}