extern crate dotenv;
#[macro_use] extern crate diesel;
pub mod schema;
pub mod models;
use tonic::{transport::Server};
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::authentication_service::AuthenticationService;
use crate::authentication_service::authentication::authentication_server::{AuthenticationServer};
mod authentication_service;

fn new_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create postgres pool");
    pool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = new_pool();
    env_logger::init();
    let addr = "[::1]:50051".parse()?;


    let authentication = AuthenticationService::new(pool.clone());

    Server::builder()
        .add_service(AuthenticationServer::new(authentication))
        .serve(addr)
        .await?;

    Ok(())
}