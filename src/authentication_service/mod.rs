mod email_identity_service;
mod handler;
mod token_service;

use tonic::{Request, Response, Status};
use authentication::authentication_server::{Authentication};
use authentication::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ForgotPasswordRequest, ForgotPasswordResponse, ChangePasswordRequest, ChangePasswordResponse, UserRequest, UserResponse};
use crate::Pool;

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub struct AuthenticationService {
    pool: Pool,
}

impl AuthenticationService {
    pub fn new(pool: Pool) -> AuthenticationService {
        AuthenticationService{
            pool,
        }
    }
}

#[tonic::async_trait]
impl Authentication for AuthenticationService {

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        handler::login::respond(&self.pool, request)
    }

    async fn register(&self, request: Request<RegisterRequest>,) -> Result<Response<RegisterResponse>, Status> {
        handler::register::respond(&self.pool, request).await
    }

    async fn forgot_password(&self, request: Request<ForgotPasswordRequest>) -> Result<Response<ForgotPasswordResponse>, Status> {
        handler::forgot_password::respond(request)
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>,) -> Result<Response<ChangePasswordResponse>, Status> {
        handler::change_password::respond(&self.pool, request)
    }

    async fn me(&self, request: Request<UserRequest>) -> Result<Response<UserResponse>, Status> {
        handler::me::respond(&self.pool, request)
    }

}