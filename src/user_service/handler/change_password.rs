use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::Pool;
use super::super::{ChangePasswordRequest, ChangePasswordResponse};
use super::super::token_service;
use super::super::email_identity_service::EmailIdentityService;

pub fn respond(pool: &Pool, request: Request<ChangePasswordRequest>,) -> Result<Response<ChangePasswordResponse>, Status> {
    println!("Got change password request: {:?}", request);
    token_service::verify(request, |data, user_id| {
        let connection = pool.get();
        if connection.is_err() {
            return Err(Status::data_loss("Could not connect to database"));
        }
        let connection = connection.unwrap();
        let (is_match, email) = EmailIdentityService::matches_user_id(&connection, &data.current_password, &user_id)?;
        if !is_match {
            return Err(Status::unauthenticated("The credentials do not match"));
        }
        let pool = pool.clone();
        tokio::spawn(async move {
            let uuid = Uuid::parse_str(&user_id).unwrap();
            let password_service = EmailIdentityService::new();
            futures::executor::block_on(password_service.store_email_identity(&pool, &data.new_password, &uuid, &email))
        });
        Ok(Response::new(ChangePasswordResponse{}))
    })
}