use tonic::{Request, Response, Status};
use authentication::authentication_server::{Authentication};
use authentication::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ForgotPasswordRequest, ForgotPasswordResponse, ChangePasswordRequest, ChangePasswordResponse};
use diesel::prelude::*;
use crate::models::User;
use crate::Pool;
use crate::schema::users::columns::{email, first_name, last_name};

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub struct AuthenticationService {
    pool: Pool,
}

impl AuthenticationService {
    pub fn new(pool: Pool) -> AuthenticationService {
        AuthenticationService{
            pool
        }
    }
}

#[tonic::async_trait]
impl Authentication for AuthenticationService {

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got login request: {:?}", request);

        let response = authentication::LoginResponse {
            token: format!("Hello {}!", (request.into_inner().email)).into(),
        };

        Ok(Response::new(response))
    }

    async fn register(&self, request: Request<RegisterRequest>,) -> Result<Response<RegisterResponse>, Status> {
        println!("Got register request: {:?}", request);
        use super::schema::users;
        let connection = self.pool.get().unwrap();
        let user: User = diesel::insert_into(users::table)
            .values((
                email.eq(&request.into_inner().email),
                first_name.eq("Hasi"),
                last_name.eq("Hopterix")))
            .get_result(&connection)
            .expect("Error saving new user.");
        let response = authentication::RegisterResponse {
            token: format!("Hello {}!", user.email),
            user_id: format!("uuuid-test {:?}", user.id),
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
            token: format!("Hello {}!", request.into_inner().new_password),
        };

        Ok(Response::new(response))
    }

}