use tonic::{Request, Response, Status};

use authentication::authentication_server::{Authentication};
use authentication::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ForgotPasswordRequest, ForgotPasswordResponse, ChangePasswordRequest, ChangePasswordResponse};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[derive(Debug, Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got login request: {:?}", request);

        let response = authentication::LoginResponse {
            token: format!("Hello {}!", request.into_inner().email).into(),
        };

        Ok(Response::new(response))
    }

    async fn register(&self, request: Request<RegisterRequest>,) -> Result<Response<RegisterResponse>, Status> {
        println!("Got register request: {:?}", request);
        let response = authentication::RegisterResponse {
            token: format!("Hello {}!", request.into_inner().email).into(),
            user_id: format!("uuuid-test"),
        };
        Ok(Response::new(response))
    }

    async fn forgot_password(&self, request: Request<ForgotPasswordRequest>,) -> Result<Response<ForgotPasswordResponse>, Status> {
        println!("Got forgot_password request: {:?}", request);
        let response = authentication::ForgotPasswordResponse {
            success: true
        };
        Ok(Response::new(response))
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>,) -> Result<Response<ChangePasswordResponse>, Status> {
        println!("Got change password request: {:?}", request);
        let response = authentication::ChangePasswordResponse {
            token: format!("Hello {}!", request.into_inner().new_password).into(),
        };

        Ok(Response::new(response))
    }

}