use tonic::{Request, Response, Status};
use crate::{Pool, PooledConnection};
use super::super::authentication::{LoginRequest, LoginResponse};
use diesel::prelude::*;
use crate::authentication_service::email_identity::EmailIdentityService;

type ResponseResult = Result<Response<LoginResponse>, Status>;

pub fn respond(pool: &Pool, request: Request<LoginRequest>) -> ResponseResult {
    println!("Got login request: {:?}", request);
    let request_data = request.into_inner();
    let connection = pool.get().unwrap();
    match_user_with_hash(&connection, &request_data.email, &request_data.password)
}

fn match_user_with_hash(connection: &PooledConnection, user_email: &str, password: &str) -> ResponseResult {
    use crate::schema::email_identities::dsl::*;
    match email_identities.filter(email.eq(user_email)).filter(created_at.eq(updated_at)).select(hash).first::<String>(connection) {
        Ok(password_hash) => match_password_with_hash(password, password_hash),
        Err(e) => {
            println!("error in login: {}", e);
            err()
        }
    }
}

fn match_password_with_hash(password: &str, hash: String) -> ResponseResult {
    match EmailIdentityService::matches(password, hash) {
        true => {
            let response = LoginResponse {
                token: format!("Hello {}!", "das ist super"),
            };
            Ok(Response::new(response))
        },
        false => err()
    }
}

fn err() -> ResponseResult {
    Err(Status::unauthenticated("Credentials don't match or user does not exist."))
}