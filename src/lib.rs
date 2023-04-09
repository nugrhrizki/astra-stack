use std::{env, net::SocketAddr, time::Duration};

use axum::Router;
use sqlx::postgres::PgPoolOptions;

pub mod utils;

pub async fn run(router: Router) {
    // create the address
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let ip: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    axum::Server::bind(&ip)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

pub async fn connect_database() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
