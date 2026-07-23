use dotenv::dotenv;
use std::sync::Arc;

// mod datbase; implement later
mod handlers;
mod models;
mod router;
mod schemas;
mod types;

use crate::router::game::create_router;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    // Connect to Postgres
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to DB succesfully");
            pool
        }
        Err(err) => {
            println!("Failed to connect to DB: {}", err);
            std::process::exit(1);
        }
    };

    // build our application with a single route
    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
