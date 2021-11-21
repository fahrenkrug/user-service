use tonic::{transport::Server};
use crate::authentication_service::AuthenticationService;
use crate::authentication_service::authentication::authentication_server::{AuthenticationServer};

mod authentication_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let authentication = AuthenticationService::default();

    Server::builder()
        .add_service(AuthenticationServer::new(authentication))
        .serve(addr)
        .await?;

    Ok(())
}