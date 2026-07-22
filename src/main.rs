use axum::{Router, routing::get};
mod datbase;
mod routes;
use datbase::connect_to_pg_database;

#[tokio::main]
async fn main() {
    // Connect to Postgres
    connect_to_pg_database().await;

    // build our application with a single route
    let app = Router::new().route("/api", get(routes::hello_world));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
