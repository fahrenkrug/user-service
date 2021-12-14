extern crate dotenv;
#[macro_use] extern crate diesel;
pub mod models;
pub mod schema;
use tonic::{transport::Server};
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::time::Duration;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::authentication_service::AuthenticationService;
use authentication_service::authentication::authentication_server::AuthenticationServer;

mod authentication_service;

fn new_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().min_idle(Some(1)).connection_timeout(Duration::from_secs(60 * 10)).build(manager).expect("Failed to create postgres pool")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let pool = new_pool();

    let addr = "[::1]:50051".parse()?;

    let authentication_service = AuthenticationService::new(pool.clone());

    let authentication_server = AuthenticationServer::new(authentication_service);
    let authentication_server = tonic_web::config().allow_origins(vec!["127.0.0.1"])
        .enable(authentication_server);

    Server::builder()
        .accept_http1(true)
        .add_service(authentication_server)
        .serve(addr)
        .await?;

    Ok(())
}