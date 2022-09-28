mod email_identity_service;
mod handler;
mod token_service;

use tonic::{Request, Response, Status};
use user::user_server::{User};
use user::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ForgotPasswordRequest, ForgotPasswordResponse, ChangePasswordRequest, ChangePasswordResponse, UserRequest, UserResponse};
use crate::Pool;

pub mod user {
    tonic::include_proto!("user");
}

pub struct UserService {
    pool: Pool,
}

impl UserService {
    pub fn new(pool: Pool) -> UserService {
        UserService{
            pool,
        }
    }
}

#[tonic::async_trait]
impl User for UserService {

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