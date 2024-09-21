extern crate axum;
extern crate serde;
extern crate shuttle_axum;
extern crate shuttle_runtime;
extern crate sqlx;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use shuttle_runtime::SecretStore;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn hello_world(State(pool): State<Pool<MySql>>) -> Json<Greeting> {
    let result = sqlx::query_scalar("SELECT CONCAT('Hello, ', version()) AS greeting")
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| "Hello, world!".to_string());

    Json(Greeting { message: result })
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Set up database connection
    let database_url = secrets
        .get("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let router = Router::new().route("/", get(hello_world)).with_state(pool);

    Ok(router.into())
}
