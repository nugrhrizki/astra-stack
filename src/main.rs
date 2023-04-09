use std::sync::Arc;

use astra_stack::utils::{Response, ResponseTuple};
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    // load env variables
    dotenv::dotenv().ok();

    // connect to database
    let pool = astra_stack::connect_database().await.unwrap();

    // create the Context
    let context = Arc::new(AppState { pool });

    let routes = Router::new()
        .route("/", get(heartbeat))
        .route("/hello/:name", get(greet))
        .route("/detak-db", get(detak_db))
        .with_state(context);

    astra_stack::run(routes).await;
}

async fn heartbeat() -> &'static str {
    "OK"
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn detak_db(State(state): State<Arc<AppState>>) -> Result<ResponseTuple, ResponseTuple> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|_| Response::internal_server_error().to_tuple())?;

    let row = sqlx::query!("SELECT 1 + 1 AS result")
        .fetch_one(&mut conn)
        .await
        .map_err(|_| Response::internal_server_error().to_tuple())?;

    let result = Response::ok(Some(serde_json::json!(row.result))).to_tuple();

    Ok(result)
}
